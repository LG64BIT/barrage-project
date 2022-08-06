use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{models::user::User, utils::AppState};

pub async fn handle(state: Data<AppState>, id: Path<String>) -> impl Responder {
    let connection = state.get_pg_connection();
    match User::get_by_id(&connection, &id) {
        Ok(user) => return HttpResponse::Ok().json(user),
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}
