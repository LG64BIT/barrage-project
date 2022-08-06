use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::product::Product;
use crate::{schema::products, utils::AppState};
use actix_web::{web::Data, HttpResponse, Responder};

pub async fn handle(state: Data<AppState>) -> impl Responder {
    let connection = state.get_pg_connection();
    let list_of_products = match products::table
        .select(products::all_columns)
        .load::<Product>(&connection)
    {
        Ok(products) => products,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    let real_products = Product::to_real_products(&list_of_products);
    HttpResponse::Ok().json(real_products)
}
