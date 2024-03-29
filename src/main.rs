use std::env;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;

pub mod errors;
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
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();

    dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(utils::initialize()))
            .service(web::scope("/").configure(routes::router))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
