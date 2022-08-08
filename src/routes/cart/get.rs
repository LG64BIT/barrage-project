use actix_web::cookie::Cookie;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

use crate::models::cart::Cart;

pub async fn handle(req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let cookie = match req.cookie("cart") {
        Some(c) => c,
        None => {
            let cart = Cart::new();
            Cookie::new(
                "cart",
                serde_json::to_string(&cart).expect("Failed parsing cart to string!"),
            )
        }
    };
    let cart = match serde_json::from_str::<Cart>(&cookie.value().to_string()) {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    };
    Ok(HttpResponse::Ok().json(cart))
}
