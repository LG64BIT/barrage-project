use crate::errors::ShopError;
use crate::models::product::{InsertableProduct, NewProduct, Product};
use crate::models::user::User;
use crate::utils::AppState;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::{web::Data, HttpResponse};
use validator::Validate;

pub async fn handle(
    state: Data<AppState>,
    product: Json<NewProduct>,
    req: HttpRequest,
) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    if !user.is_admin {
        return Ok(HttpResponse::Forbidden().json("No admin privileges!"));
    }
    product.validate()?;
    let connection = state.get_pg_connection()?;
    let insertable_product = InsertableProduct::new(product.into_inner());
    Product::insert(&connection, insertable_product)?;
    Ok(HttpResponse::Ok().finish())
}
