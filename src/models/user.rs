use crate::diesel::prelude::*;
use crate::errors::ShopError;
use crate::jwt::{self, UserClaims};
use crate::schema::users;
use actix_web::HttpRequest;
use bcrypt::verify;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl User {
    //Get user by email from database
    fn get_by_email(connection: &PgConnection, email: &str) -> Result<Self, ShopError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::email.eq(email))
            .first::<Self>(connection)?;
        Ok(User {
            id: result.id.clone(),
            email: result.email.clone(),
            password: result.password.clone(),
            is_admin: result.is_admin.clone(),
            created_at: result.created_at.clone(),
            updated_at: result.updated_at.clone(),
        })
    }

    pub fn get_by_id(connection: &PgConnection, id: &str) -> Result<Self, ShopError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::id.eq(id))
            .first::<Self>(connection)?;
        Ok(User {
            id: result.id.clone(),
            email: result.email.clone(),
            password: result.password.clone(),
            is_admin: result.is_admin.clone(),
            created_at: result.created_at.clone(),
            updated_at: result.updated_at.clone(),
        })
    }

    pub fn update_email(
        &mut self,
        connection: &PgConnection,
        new_email: &str,
    ) -> Result<usize, ShopError> {
        let affected = diesel::update(users::table)
            .filter(users::id.eq(&self.id))
            .set(users::email.eq(&new_email))
            .execute(connection)?;
        self.email = new_email.to_string();
        Ok(affected)
    }

    pub fn update_pass(
        &mut self,
        connection: &PgConnection,
        new_pass: &str,
    ) -> Result<usize, ShopError> {
        let affected = diesel::update(users::table)
            .filter(users::id.eq(&self.id))
            .set(users::password.eq(&new_pass))
            .execute(connection)?;
        self.password = new_pass.to_string();
        Ok(affected)
    }

    pub fn is_available_email(connection: &diesel::PgConnection, email: &str) -> bool {
        let result = User::get_by_email(connection, email);
        if result.is_err() {
            return true;
        }
        return false;
    }

    pub fn create(connection: &PgConnection, mut new_user: NewUser) -> Result<User, ShopError> {
        if !User::is_available_email(&connection, &new_user.email) {
            return Err(ShopError::AlreadyExistsError);
        }
        new_user.password = bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST)?;
        Ok(new_user.create(&connection)?)
    }

    pub fn update(
        &mut self,
        connection: &PgConnection,
        mut new_user: NewUser,
    ) -> Result<(), ShopError> {
        if !User::is_available_email(&connection, &new_user.email) {
            return Err(ShopError::AlreadyExistsError);
        }
        new_user.password = bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST)?;
        new_user.update(connection, &self.id)?;
        self.email = new_user.email;
        self.password = new_user.password;
        Ok(())
    }

    ///removing all user data, but not removing column in database, in case some dependencies depend on it
    /// also, update is much faster than delete performance wise
    pub fn anonymize(connection: &PgConnection, id: &str) -> Result<usize, ShopError> {
        Ok(diesel::update(users::table)
            .set((
                users::email.eq(Uuid::new_v4().to_string()),
                users::password.eq(Uuid::new_v4().to_string()),
            ))
            .filter(users::id.eq(id))
            .execute(connection)?)
    }

    pub fn authenticate(
        connection: &PgConnection,
        email: &str,
        password: &str,
    ) -> Result<(User, String), ShopError> {
        let user = User::get_by_email(&connection, &email)?;
        if !verify(password, &user.password)? {
            return Err(ShopError::InvalidInput);
        }
        let token = user.generate_jwt()?;
        Ok((user, token))
    }

    pub fn generate_jwt(&self) -> Result<String, ShopError> {
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

    pub fn is_logged(req: &HttpRequest) -> Result<User, ShopError> {
        let user_jwt = match req.headers().get("jwt") {
            Some(jwt) => jwt.to_str()?,
            None => return Err(ShopError::InvalidInput),
        };
        Ok(jwt::verify(String::from(user_jwt))?)
    }
}

//insert into database
#[derive(Insertable, Debug, Deserialize, validator::Validate, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH"))]
    pub password: String,
}

impl NewUser {
    pub fn create(&self, connection: &PgConnection) -> Result<User, ShopError> {
        Ok(diesel::insert_into(users::table)
            .values(self)
            .get_result::<User>(connection)?)
    }

    pub fn update(&self, connection: &PgConnection, user_id: &str) -> Result<usize, ShopError> {
        Ok(diesel::update(users::table)
            .set(self)
            .filter(users::id.eq(user_id))
            .execute(connection)?)
    }
}
