use actix_web::{web::Json, HttpResponse, cookie::Cookie};

use crate::{models::user::{NewUser, UserError, User}, utils};



pub async fn handle(user: Json<NewUser>) -> Result<HttpResponse, UserError> {
    let connection = utils::establish_connection();

    match User::authenticate(&connection, &user.email, &user.password) {
        Ok((valid, token)) => {
            let cookie = Cookie::new("jwt", token.clone());
            Ok(HttpResponse::Ok()
                //.append_header(("jwt", token))
                .cookie(cookie)
                .json(valid))
        }
        Err(e) => Err(e),
    }
}