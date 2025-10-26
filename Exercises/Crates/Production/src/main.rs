use axum::{
    routing::{get, post, put, delete},
    Router, Json, extract::{Path, State},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://user:password@localhost/mydb").await?;

    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
