use actix_web::{
    web::{self, ServiceConfig},
    Responder,
};

pub mod cart;
pub mod products;
pub mod users;

pub async fn test() -> impl Responder {
    "Noice".to_string()
}

pub fn router(conf: &mut ServiceConfig) {
    //conf.service(web::resource("/test").route(web::get().to(test)));
    conf.service(web::resource("/self").route(web::get().to(users::index::handle)));
    conf.service(web::resource("/register").route(web::post().to(users::register::handle)));
    conf.service(web::resource("/login").route(web::post().to(users::login::handle)));
    conf.service(
        web::resource("/user/{id}")
            .route(web::put().to(users::update::handle))
            .route(web::get().to(users::get::handle))
            .route(web::delete().to(users::delete::handle))
    );
    conf.service(web::resource("/addProduct").route(web::post().to(products::add::handle)));
    conf.service(web::resource("/products").route(web::get().to(products::all::handle)));
    conf.service(
        web::resource("/products/{product_id}").route(web::get().to(products::single::handle)),
    );
    conf.service(web::resource("/cart/{product_id}").route(web::post().to(cart::add::handle)));
    conf.service(
        web::resource("/cart")
            .route(web::get().to(cart::get::handle))
            .route(web::delete().to(cart::empty::handle)),
    );
}
