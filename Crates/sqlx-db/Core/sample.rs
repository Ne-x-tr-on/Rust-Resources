use axum::{
    routing::{post},
    Router,
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct NewUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db_url = "postgres://myuser:mypass@localhost:5432/mydb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    let app = Router::new()
        .route("/users", post(add_user)) // POST endpoint
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

// POST /users
async fn add_user(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<NewUser>,
) -> Json<User> {
    let row = sqlx::query_as!(
        User,
        r#"INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"#,
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(row)
}
