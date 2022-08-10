use crate::{
    errors::ShopError,
    models::user::{NewUser, User},
    utils::AppState,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use validator::Validate;

pub async fn handle(
    state: Data<AppState>,
    new_user: Json<NewUser>,
) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let new_user = new_user.into_inner();
    new_user.validate()?;
    let created = User::create(&connection, new_user)?;
    Ok(HttpResponse::Ok().json(created))
}
