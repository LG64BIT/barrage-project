use crate::errors::ShopError;
use crate::models::cart::Cart;
use crate::models::order::Order;
use crate::models::user::User;
use crate::utils::AppState;
use actix_web::HttpRequest;
use actix_web::{web::Data, HttpResponse};

pub async fn handle(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    let cart = Cart::get(&req)?;
    let connection = state.get_pg_connection()?;
    let response = Order::make_order(&connection, &user.id, cart)?;
    Ok(response)
}
