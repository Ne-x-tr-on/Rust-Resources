use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Picture {
    pub id: Uuid,
    pub title: String,
    pub event_type: String,
    pub file_path: String,
}

#[derive(Deserialize)]
pub struct NewPicture {
    pub title: String,
    pub event_type: String,
    pub file_path: String,
}

// Build the router with routes and database state
pub fn app_router(db_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/upload", post(upload_picture))
        .route("/pictures/:event_type", get(get_pictures_by_event))
        .with_state(db_pool)
}

// Handler to upload a new picture
async fn upload_picture(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NewPicture>,
) -> Json<String> {
    let id = Uuid::new_v4();

    let result = sqlx::query!(
    "INSERT INTO course.pictures (id, title, event_type, file_path) VALUES ($1, $2, $3, $4)",
    id,
    payload.title,
    payload.event_type,
    payload.file_path
)
.execute(&pool)
.await?;


    match result {
        Ok(_) => Json(format!("✅ Picture added with id: {}", id)),
        Err(err) => Json(format!("❌ Database error: {}", err)),
    }
}

// Handler to fetch pictures by event type
async fn get_pictures_by_event(
    State(pool): State<Pool<Postgres>>,
    Path(event_type): Path<String>,
) -> Json<Vec<Picture>> {
    let records = sqlx::query_as!(
        Picture,
        "SELECT id, title, event_type, file_path FROM course.pictures WHERE event_type = $1",
        event_type
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(records)
}
