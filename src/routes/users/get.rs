use actix_web::{web::Path, Responder, HttpResponse};

use crate::{utils, models::user::User};



pub async fn handle(id: Path<String>) -> impl Responder {
    let connection = utils::establish_connection();
    match User::get(&connection, &id) {
        Ok(user) => return HttpResponse::Ok().json(user),
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}