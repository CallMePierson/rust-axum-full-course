#![allow(unused)]

use std::net::SocketAddr;

use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_start = Router::new().route(
        "/start",
        get(handler_start),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> LISTENING on http://{addr}\n");

    axum::Server::bind(&addr).serve(routes_start.into_make_service()).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct StartParams {
    name: Option<String>,
}

async fn handler_start(Query(params): Query<StartParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_start - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello {name}!!"))
}