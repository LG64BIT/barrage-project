use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;

pub mod jwt;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(utils::initialize()))
            .service(web::scope("/").configure(routes::router))
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
