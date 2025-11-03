use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
}

async fn create_post(Json(payload): Json<Post>) -> Json<Post> {
    println!("Received post: {:?}", payload);
    Json(payload)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(create_post));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
