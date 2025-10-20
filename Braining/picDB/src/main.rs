mod db;
mod routes;

use axum::{Router};
use routes::app_router;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer};
use db::connect_db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = connect_db().await;

    let app = app_router(db).layer(CorsLayer::permissive());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("🚀 Server running on http://{}", addr);
   let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
   axum::serve(listener,app).await.unwrap();
}
