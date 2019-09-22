use std::sync::Arc;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use futures::future::Future;

use super::schema::query::*;
use actix_web::{web, Error, HttpResponse};

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(
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

pub fn get_schema() -> Arc<Schema> {
    Arc::new(create_schema())
}
