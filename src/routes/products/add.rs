use crate::diesel::RunQueryDsl;
use crate::models::product::{InsertableProduct, NewProduct};
use crate::models::user::User;
use crate::{schema::products, utils::AppState};
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::{web::Data, HttpResponse, Responder};
use validator::Validate;

pub async fn handle(
    state: Data<AppState>,
    product: Json<NewProduct>,
    req: HttpRequest,
) -> impl Responder {
    let user = match User::is_logged(&req) {
        Ok(user) => user,
        Err(e) => return HttpResponse::Forbidden().json(e.to_string()),
    };
    if !user.is_admin {
        return HttpResponse::Forbidden().json("No admin privileges!");
    }
    match product.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }
    let connection = state.get_pg_connection();
    let insertable_product = InsertableProduct::new(product.into_inner());
    match diesel::insert_into(products::table)
        .values(insertable_product)
        .execute(&connection)
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
