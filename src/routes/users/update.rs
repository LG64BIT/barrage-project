use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};

use crate::{
    errors::ShopError,
    models::user::{NewUser, User},
    utils::AppState,
};

pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    new_user: Json<NewUser>,
    id: Path<String>,
) -> Result<HttpResponse, ShopError> {
    let current_user = User::is_logged(&req)?;
    let id = id.into_inner();
    if id != current_user.id && !current_user.is_admin {
        return Err(ShopError::NoPermission(
            "You do not have permission for that action!".to_string(),
        ));
    }
    let connection = state.get_pg_connection()?;
    let mut old_user = User::get_by_id(&connection, &id)?;
    old_user.update(&connection, new_user.into_inner())?;
    Ok(HttpResponse::Ok().json(old_user))
}
