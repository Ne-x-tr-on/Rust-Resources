use axum::{
    Router,
    routing::get
};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    // Build your application with a route
    let app = routes::main_router();

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running at http://{}", addr);

    // Run it with hyper
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
