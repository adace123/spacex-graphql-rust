extern crate juniper;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use std::io::Result;

mod schema;
use schema::*;

mod spacex_api;
use spacex_api::*;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let schema = get_schema();
    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::CONTENT_TYPE,
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                    ])
                    .supports_credentials(),
            )
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
