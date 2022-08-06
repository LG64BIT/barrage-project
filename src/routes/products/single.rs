use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::product::Product;
use crate::{schema::products, utils::AppState};
use actix_web::web::Path;
use actix_web::{web::Data, HttpResponse, Responder};

pub async fn handle(state: Data<AppState>, product_id: Path<String>) -> impl Responder {
    let connection = state.get_pg_connection();
    let product = match products::table
        .select(products::all_columns)
        .filter(products::id.eq(product_id.into_inner()))
        .first::<Product>(&connection)
    {
        Ok(product) => product,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    let real_products = product.to_real_product();
    HttpResponse::Ok().json(real_products)
}
