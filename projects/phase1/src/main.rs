use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

// Define a struct we’ll send to the frontend
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    is_active: bool,
}

// Our handler that returns the struct as JSON
async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "Newton Manyeki".to_string(),
        email: "newton@example.com".to_string(),
        is_active: true,
    };

    Json(user)
}

#[tokio::main]
async fn main() {
    // Build the router
    let app = Router::new().route("/user", get(get_user));

    // Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // Run the server
    axum::serve(listener,app).await.unwrap();
}
