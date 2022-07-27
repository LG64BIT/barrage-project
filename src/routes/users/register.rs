use crate::{
    models::user::{NewUser, UserError},
    utils
};
use actix_web::{
    web::Json,
    HttpResponse
};
use validator::Validate;

pub async fn handle(user: Json<NewUser>) -> Result<HttpResponse, UserError> {
    let connection = utils::establish_connection();
    match user.validate() {
        Ok(_) => (),
        Err(_) => return Err(UserError::InvalidCredentials),
    };
    match NewUser::create(&connection, &user.email, &user.password) {
        Ok(created) => Ok(HttpResponse::Ok().json(created)),
        Err(e) => Err(e),
    }
}