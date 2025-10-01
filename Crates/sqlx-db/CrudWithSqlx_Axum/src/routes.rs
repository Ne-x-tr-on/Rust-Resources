use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

use crate::models::{User, NewUser, UpdateUser};

// GET /users
// GET /users
pub async fn list_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let rows: Result<Vec<User>, sqlx::Error> =
        sqlx::query_as!(User, "SELECT id, name, email FROM users ORDER BY id")
            .fetch_all(&pool)   // ✅ use `pool` instead of `state.db`
            .await;

    match rows {
        Ok(users) => Json(users).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


// GET /users/{id}
pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let row = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// POST /users
pub async fn add_user(State(pool): State<PgPool>, Json(payload): Json<NewUser>) -> impl IntoResponse {
    let row = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await;

    match row {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// PUT /users/{id}
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let row = sqlx::query_as!(
        User,
        "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 RETURNING id, name, email",
        payload.name,
        payload.email,
        id
    )
    .fetch_optional(&pool)
    .await;

    match row {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// DELETE /users/{id}
pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => (StatusCode::OK, format!("User {id} deleted")).into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
