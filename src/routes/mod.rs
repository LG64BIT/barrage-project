use actix_web::{
    web::{self, ServiceConfig},
    Responder,
};

pub mod users;

pub async fn test() -> impl Responder {
    "Noice".to_string()
}

pub fn router(conf: &mut ServiceConfig) {
    //conf.service(web::resource("/test").route(web::get().to(test)));
    conf.service(web::resource("/self").route(web::get().to(users::index::handle)));
    conf.service(web::resource("/register").route(web::post().to(users::register::handle)));
    conf.service(web::resource("/login").route(web::post().to(users::login::handle)));
    conf.service(web::resource("/logout").route(web::get().to(users::logout::handle)));
    conf.service(
        web::resource("/user/{id}")
            .route(web::put().to(users::update::handle))
            .route(web::get().to(users::get::handle)),
    );
}
