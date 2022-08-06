use actix_web::{cookie::Cookie, HttpResponse, Responder};

pub async fn handle() -> impl Responder {
    let mut removal_cookie = Cookie::new("cart", "");
    removal_cookie.make_removal();
    let mut resp = HttpResponse::Ok().finish();
    resp.add_removal_cookie(&removal_cookie).unwrap();
    resp
}
