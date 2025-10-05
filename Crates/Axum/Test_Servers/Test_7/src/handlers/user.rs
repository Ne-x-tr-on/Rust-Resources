use axum::response::Json;
use axum::http::StatusCode;
use serde_json::json;

pub async fn get_user() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({"message": "Fetched users successfully"}))
    )
}

pub async fn create_user() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(json!({"message": "User created successfully"}))
    )
}
