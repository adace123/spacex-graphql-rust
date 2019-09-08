extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
use futures::future::Future;
use std::io::Result;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};

extern crate juniper;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use schema::{create_schema, Context, Schema};

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let ctx = Context {
            base_url: "https://api.spacexdata.com/v3".to_owned(),
        };
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|content| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(content))
    })
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();
    let schema = std::sync::Arc::new(create_schema());
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
