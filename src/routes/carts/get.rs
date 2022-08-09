use actix_web::HttpRequest;
use actix_web::HttpResponse;

use crate::errors::ShopError;
use crate::models::cart::Cart;

pub async fn handle(req: HttpRequest) -> Result<HttpResponse, ShopError> {
    let cart = Cart::get(&req)?;
    Ok(HttpResponse::Ok().json(cart))
}
