use axum::{
    routing::{get, post},
    response::IntoResponse,
    http::StatusCode,
    extract::{Path, State},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::error::Error;
use tokio;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Server starting...");

    // Create router
    let app = Router::new().route("/", get(sample_handler));

    // Define address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Running on http://{}", addr);

    // Run server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn sample_handler() -> impl IntoResponse {
    let user = User {
        username: "Newton".to_string(),
        age: 19,
    };

    println!("{:?}", user);

    // Return JSON response with status code
    (StatusCode::OK, Json(user))
}
