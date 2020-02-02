#[macro_use]
extern crate diesel;
mod graphql_schema;
mod schema;

use crate::graphql_schema::{create_schema, Schema};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use async_std::io::Result;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphql1").route(web::get().to(graphql1)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn graphql(st: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let res = data.execute(&st, &());
    let user = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(user)
}

async fn graphql1() -> impl Responder {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
