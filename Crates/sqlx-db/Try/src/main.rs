pub mod db;
pub mod models;


use axum::{
    Router,
    routing::get,
    Json,
    response::IntoResponse,
    http::StatusCode,
};

use db::create_connection;

#[tokio::main]
async fn main() {
    let pool = create_connection().await;

    let app = Router::new().route("/", get(|| async { "✅ Database connected!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}


// use axum::{Router, routing::get, extract::State};
// use std::sync::Arc;

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
// Atomic Reference Counted
//     let pool = Arc::new(create_connection().await?);

//     let app = Router::new()
//         .route("/health", get(health_check))
//         .with_state(pool.clone());

//     println!("✅ Server running...");
//     Ok(())
// }

// async fn health_check(State(pool): State<Arc<PgPool>>) -> String {
//     if sqlx::query!("SELECT 1").fetch_one(&*pool).await.is_ok() {
//         "Database OK ✅".to_string()
//     } else {
//         "Database Error ❌".to_string()
//     }
// }

