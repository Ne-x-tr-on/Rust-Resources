use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(|| async { "Newton With Axum" }));

    // Create a TCP listener
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    // Start the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
