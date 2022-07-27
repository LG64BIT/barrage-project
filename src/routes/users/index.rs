use actix_web::{HttpRequest, Responder, HttpResponse};

use crate::jwt::verify;



pub async fn handle(req: HttpRequest) -> impl Responder {
    let user_cookie = match req.cookie("jwt") {
        Some(cookie) => cookie,
        None => return HttpResponse::Forbidden().finish(),
    };
    let user_jwt = user_cookie.value().to_string();
    match verify(user_jwt) {
        Ok(user) => return HttpResponse::Ok().json(user),
        Err(_) => return HttpResponse::Forbidden().finish(),
    };
}