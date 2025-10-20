use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Book, Member, Loan};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
}

// Create router
pub fn app_router(pool: PgPool) -> Router {
    Router::new()
        .route("/books", post(add_book))
        .route("/books/:id", get(get_book_by_id))
        .with_state(pool)
}

// Add a book
async fn add_book(
    State(pool): State<PgPool>,
    Json(payload): Json<NewBook>,
) -> Json<String> {
    let id = Uuid::new_v4();
    let result = sqlx::query!(
        "INSERT INTO library.books (id, title, author) VALUES ($1, $2, $3)",
        id,
        payload.title,
        payload.author
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(format!("Book added with ID: {}", id)),
        Err(err) => Json(format!("Error: {}", err)),
    }
}

// Get a book by ID
async fn get_book_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<Option<Book>> {
    let record = sqlx::query_as!(
        Book,
        "SELECT id, title, author, published_at FROM library.books WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    Json(record)
}
