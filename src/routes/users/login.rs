use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    errors::ShopError,
    models::user::{NewUser, User},
    utils::AppState,
};

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let (valid, token) = User::authenticate(&connection, &user.email, &user.password)?;
    Ok(HttpResponse::Ok().append_header(("jwt", token)).json(valid))
}
