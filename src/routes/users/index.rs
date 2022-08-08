use actix_web::{HttpRequest, HttpResponse};

use crate::{errors::ShopError, models::user::User};

pub async fn handle(req: HttpRequest) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    Ok(HttpResponse::Ok().json(user))
}
