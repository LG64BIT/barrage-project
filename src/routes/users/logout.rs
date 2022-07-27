use actix_web::{Responder, cookie::Cookie, HttpResponse};



pub async fn handle() -> impl Responder {
    let mut cookie = Cookie::build("jwt", "0").finish();
    cookie.make_removal();

    let mut response = HttpResponse::Ok().finish();
    response
        .add_removal_cookie(&cookie)
        .unwrap_or_else(|_| response = HttpResponse::BadGateway().finish());
        response

}