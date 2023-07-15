#![allow(unused)]

use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

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

async fn handler_start() -> impl IntoResponse {
    println!("->> {:<12} - handler_start", "HANDLER");
    Html("Hello World!!")
}