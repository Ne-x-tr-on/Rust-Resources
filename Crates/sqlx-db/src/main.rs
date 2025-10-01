use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{Path, State},
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
    .route("/users", get(list_users))
    .route("/users", post(add_user))
    .route("/users/{id}", put(update_user))
    .route("/users/{id}", delete(delete_user))
    .with_state(pool);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

// GET /users
async fn list_users(State(pool): State<sqlx::PgPool>) -> Json<Vec<User>> {
    let rows = sqlx::query_as!(
        User,
        r#"SELECT id, name, email FROM users ORDER BY id"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(rows)
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

// PUT /users/:id
async fn update_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<NewUser>,
) -> Json<User> {
    let row = sqlx::query_as!(
        User,
        r#"UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email"#,
        payload.name,
        payload.email,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(row)
}

// DELETE /users/:id
async fn delete_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Json<String> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
        .unwrap();

    Json(format!("✅ User with id {} deleted", id))
}
