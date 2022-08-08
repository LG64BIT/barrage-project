use actix_web::{
    web::{Data, Path},
    HttpResponse,
};

use crate::{errors::ShopError, models::user::User, utils::AppState};

pub async fn handle(state: Data<AppState>, id: Path<String>) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let user = User::get_by_id(&connection, &id)?;
    Ok(HttpResponse::Ok().json(user))
}
