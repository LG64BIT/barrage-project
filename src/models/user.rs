use crate::diesel::prelude::*;
use crate::jwt::{self, UserClaims};
use crate::{schema::users, utils};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use bcrypt::verify;
use chrono::NaiveDateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};

pub const MIN_PASSWORD_LENGTH: u8 = 8;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Display, Clone)]
pub enum UserError {
    AlreadyExistsError,
    NotFoundError,
    ConnectionError,
    InvalidCredentials,
    NoPermission,
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::AlreadyExistsError => StatusCode::ALREADY_REPORTED,
            UserError::NotFoundError => StatusCode::NOT_FOUND,
            UserError::ConnectionError => StatusCode::REQUEST_TIMEOUT,
            UserError::InvalidCredentials => StatusCode::BAD_REQUEST,
            UserError::NoPermission => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl User {
    //Get user by email from database
    fn get_by_email(connection: &PgConnection, email: &str) -> Result<Self, UserError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::email.eq(email))
            .load::<Self>(connection);
        if result.is_err() {
            return Err(UserError::ConnectionError);
        }
        let result = result.unwrap();
        if result.len() != 1 {
            return Err(UserError::NotFoundError);
        }
        Ok(User {
            id: result[0].id.clone(),
            email: result[0].email.clone(),
            password: result[0].password.clone(),
            is_admin: result[0].is_admin.clone(),
            created_at: result[0].created_at.clone(),
            updated_at: result[0].updated_at.clone(),
        })
    }

    pub fn get(connection: &PgConnection, id: &str) -> Result<Self, UserError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::id.eq(id))
            .load::<Self>(connection);
        if result.is_err() {
            return Err(UserError::ConnectionError);
        }
        let result = result.unwrap();
        if result.len() != 1 {
            return Err(UserError::NotFoundError);
        }
        Ok(User {
            id: result[0].id.clone(),
            email: result[0].email.clone(),
            password: result[0].password.clone(),
            is_admin: result[0].is_admin.clone(),
            created_at: result[0].created_at.clone(),
            updated_at: result[0].updated_at.clone(),
        })
    }

    pub fn update_email(
        &mut self,
        connection: &PgConnection,
        new_email: String,
    ) -> Result<usize, diesel::result::Error> {
        let affected = diesel::update(users::table)
            .filter(users::id.eq(&self.id))
            .set(users::email.eq(&new_email))
            .execute(connection);
        self.email = new_email;
        affected
    }

    pub fn is_available_email(connection: &diesel::PgConnection, email: &str) -> bool {
        let result = User::get_by_email(connection, email);
        if result.is_err() {
            return true;
        }
        return false;
    }

    //remove from database
    pub fn remove(email: &str) {
        let connection = utils::establish_connection();
        diesel::delete(users::table.filter(users::email.eq(email)))
            .execute(&connection)
            .expect("Error deleting user");
    }

    pub fn authenticate(
        connection: &PgConnection,
        email: &str,
        password: &str,
    ) -> Result<(User, String), UserError> {
        let user = User::get_by_email(&connection, &email)?;
        if !verify(password, &user.password).unwrap() {
            return Err(UserError::InvalidCredentials);
        }
        let token = user.generate_jwt();
        Ok((user, token))
    }

    pub fn generate_jwt(&self) -> String {
        crate::jwt::generate(&self)
    }

    pub fn from_jwt(claims: &UserClaims) -> Self {
        User {
            id: String::from(&claims.id),
            email: String::from(&claims.email),
            password: String::new(),
            is_admin: claims.is_admin,
            updated_at: claims.updated_at,
            created_at: claims.created_at,
        }
    }

    pub fn is_logged(req: &HttpRequest) -> Result<User, UserError> {
        let user_cookie = match req.cookie("jwt") {
            Some(cookie) => cookie,
            None => return Err(UserError::NoPermission),
        };
        let user_jwt = user_cookie.value().to_string();
        match jwt::verify(user_jwt) {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::NoPermission),
        }
    }
}

//insert into database
#[derive(Insertable, Debug, Deserialize, validator::Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH"))]
    pub password: String,
}

impl NewUser {
    pub fn create(connection: &PgConnection, email: &str, pass: &str) -> Result<User, UserError> {
        if !User::is_available_email(&connection, email) {
            return Err(UserError::AlreadyExistsError);
        }
        let user = Self {
            email: email.to_string(),
            password: bcrypt::hash(&pass, bcrypt::DEFAULT_COST).unwrap(),
        };
        match diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(connection)
        {
            Ok(user) => Ok(user),
            Err(e) => {
                dbg!(e);
                Err(UserError::ConnectionError)
            }
        }
    }
}
