use actix_web::{
    web::{Json, Path},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    models::user::{NewUser, User},
    utils::establish_connection,
};

pub async fn handle(
    req: HttpRequest,
    new_user_info: Json<NewUser>,
    id: Path<String>,
) -> impl Responder {
    let current_user = match User::is_logged(&req) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };
    let id = id.into_inner();
    if id != current_user.id && !current_user.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let connection = establish_connection();
    let old_user_info = User::get_by_id(&connection, &id);
    if old_user_info.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let mut old_user_info = old_user_info.unwrap();
    if User::is_available_email(&connection, &new_user_info.email) {
        if let Ok(_) = old_user_info.update_email(&connection, new_user_info.email.clone()) {
            return HttpResponse::Ok().finish();
        }
    }
    return HttpResponse::NotAcceptable().finish();
}
