use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::models::user::User;

pub async fn handle(req: HttpRequest) -> impl Responder {
    if let Ok(user) = User::is_logged(&req) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Forbidden().finish()
    }
}
