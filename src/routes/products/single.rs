use crate::errors::ShopError;
use crate::models::product::Product;
use crate::utils::AppState;
use actix_web::web::Path;
use actix_web::{web::Data, HttpResponse};

pub async fn handle(
    state: Data<AppState>,
    product_id: Path<String>,
) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let product = Product::get_by_id(&connection, &product_id.into_inner())?;
    let real_products = product.to_real_product();
    Ok(HttpResponse::Ok().json(real_products))
}
