use actix_web::{
    web::{Data, Path},
    HttpRequest, HttpResponse,
};

use crate::{errors::ShopError, models::user::User, utils::AppState};

pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    id: Path<String>,
) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    User::is_logged(&req)?;
    User::anonymize(&connection, &id)?;
    Ok(HttpResponse::Ok().finish())
}
