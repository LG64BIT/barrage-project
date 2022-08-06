use crate::models::cart::Cart;
use actix_web::HttpRequest;
use actix_web::{HttpResponse, Responder};

pub async fn handle(req: HttpRequest) -> impl Responder {
    let cart = match req.cookie("cart") {
        Some(c) => c,
        None => return HttpResponse::Ok().json("Empty cart"),
    };
    let cart = serde_json::from_str::<Cart>(&cart.to_string()).unwrap(); //cookie to string
    HttpResponse::Ok().json(cart)
}
