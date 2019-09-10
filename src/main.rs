extern crate pretty_env_logger;
extern crate reqwest;
extern crate spacex_api_rust_graphql;

use std::io::Result;
use actix_cors::Cors;
use spacex_api_rust_graphql::*;
use actix_web::{http, middleware, web, App, HttpServer};
extern crate juniper;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let schema = get_schema();
    
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
    .bind("127.0.0.1:8080")?
    .run()
}
