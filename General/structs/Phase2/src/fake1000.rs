use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::SocketAddr, sync::Arc};
use faker::{faker::name::en::Name, faker::internet::en::FreeEmail, Fake}; // 👈 add this
use uuid::Uuid; // for IDs

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

async fn create_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateUser>,
) -> String {
    let res = sqlx::query!(
        "INSERT INTO users (username, email) VALUES ($1, $2)",
        payload.username,
        payload.email
    )
    .execute(&*pool)
    .await
    .unwrap();

    format!("Inserted {} row(s)", res.rows_affected())
}

async fn list_users(State(pool): State<Arc<PgPool>>) -> String {
    let users = sqlx::query!("SELECT id, username, email FROM users")
        .fetch_all(&*pool)
        .await
        .unwrap();

    let mut output = String::new();
    for user in users {
        output.push_str(&format!("{:?}: {} <{}>\n", user.id, user.username, user.email));
    }
    output
}

async fn seed_users(State(pool): State<Arc<PgPool>>) -> String {
    for _ in 0..1000 {
        let username: String = Name().fake();
        let email: String = FreeEmail().fake();
        sqlx::query!("INSERT INTO users (username, email) VALUES ($1, $2)", username, email)
            .execute(&*pool)
            .await
            .unwrap();
    }
    "✅ 1000 users inserted successfully!".to_string()
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let pool = Arc::new(pool);

    let app = Router::new()
        .route("/user", post(create_user))
        .route("/users", get(list_users))
        .route("/seed", get(seed_users)) // 👈 new route
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
