#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{http,App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod works;
mod error_handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();

    //Definir cors y modelo del servidor
    let mut server = HttpServer::new(|| {
        let cors = Cors::default()
              .allowed_origin("http://localhost:4200")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b".rust-lang.org")
              })
              .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new().wrap(cors).configure(works::init_routes)
    });

    //Creación del servior
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Establecer el HOST en .env");
            let port = env::var("PORT").expect("Establecer el PORT en .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    //Correr servidor
    server.run().await
}