use crate::{models::user::NewUser, utils::AppState};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use validator::Validate;

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> HttpResponse {
    let connection = state.get_pg_connection();
    match user.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    };
    match NewUser::create(&connection, &user.email, &user.password) {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::from_error(e),
    }
}
