use crate::errors::ShopError;
use crate::models::cart::Cart;
use crate::models::cart::CartItem;
use crate::utils::AppState;
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpRequest;
use actix_web::{web::Data, HttpResponse};

pub async fn handle(
    state: Data<AppState>,
    product_id: Path<String>,
    item: Json<CartItem>,
    req: HttpRequest,
) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
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
    let mut cart = match serde_json::from_str::<Cart>(&cookie.value().to_string()) {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    };
    cart.add(
        &connection,
        product_id.to_string(),
        item.into_inner().quantity,
    )?;
    let cart = match serde_json::to_string(&cart) {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    };
    let mut cookie = Cookie::new("cart", cart);
    cookie.set_path("/");

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}
