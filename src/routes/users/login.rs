use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::user::{NewUser, User, UserError},
    utils::AppState,
};

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, UserError> {
    let connection = state.get_pg_connection();
    match User::authenticate(&connection, &user.email, &user.password) {
        Ok((valid, token)) => Ok(HttpResponse::Ok().append_header(("jwt", token)).json(valid)),
        Err(e) => Err(e),
    }
}
