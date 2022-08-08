use crate::errors::ShopError;
use crate::models::product::Product;
use crate::utils::AppState;
use actix_web::{web::Data, HttpResponse};

pub async fn handle(state: Data<AppState>) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let list_of_products = Product::get_all(&connection)?;
    let real_products = Product::to_real_products(&list_of_products);
    Ok(HttpResponse::Ok().json(real_products))
}
