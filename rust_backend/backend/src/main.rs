#![allow(unused)]

use axum::{
    routing::get,
    response::Html,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Router");
    println!("Backend Running");

    // Define the route
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello:  <strong>Backend Is Now Running...David is Proud</strong>") }),
    );

    // Corrected binding address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8800));
    println!("->> Listening on {addr}\n");

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
