use actix_web::{cookie::Cookie, HttpResponse, Responder};

pub async fn handle() -> impl Responder {
    let mut cookie = Cookie::new("cart", "");
    cookie.make_removal();
    let mut resp = HttpResponse::Ok().finish();
    resp.add_removal_cookie(&cookie).unwrap();
    resp
}
