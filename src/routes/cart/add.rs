use crate::models::cart::Cart;
use crate::models::cart::CartItem;
use crate::utils::AppState;
use actix_web::cookie::Cookie;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpRequest;
use actix_web::{web::Data, HttpResponse, Responder};

pub async fn handle(
    state: Data<AppState>,
    product_id: Path<String>,
    item: Json<CartItem>,
    req: HttpRequest,
) -> impl Responder {
    let connection = state.get_pg_connection();
    let cart = match req.cookie("cart") {
        Some(c) => c,
        None => Cookie::new::<String, String>(
            "cart".to_string(),
            serde_json::to_string(&Json(Cart::new())).unwrap(),
        ),
    };
    let mut cart = match serde_json::from_str::<Cart>(&cart.to_string()) {
        Ok(c) => c,
        Err(_) => Cart::new(),
    };
    cart.add(product_id.into_inner().to_string(), item.quantity);

    let cookie = Cookie::new::<String, String>(
        "cart".to_string(),
        serde_json::to_string(&Json(cart)).unwrap(),
    );
    HttpResponse::Ok().cookie(cookie).finish()
}
