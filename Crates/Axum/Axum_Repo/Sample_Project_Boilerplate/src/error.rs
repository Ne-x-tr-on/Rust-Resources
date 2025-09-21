use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error; // makes defining custom errors easier

// 1️⃣ Define a custom error enum
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("Payment error: {0}")]
    PaymentError(String),
}

// 2️⃣ Struct to serialize error messages
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// 3️⃣ Convert AppError into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::PaymentError(_) => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
        };

        let body = Json(ErrorResponse { error: msg });
        (status, body).into_response()
    }
}

// 4️⃣ Example usage in a handler
/*
use crate::errors::AppError;
use axum::Json;

pub async fn get_product(id: u32) -> Result<Json<Product>, AppError> {
    let product = find_product(id).ok_or(AppError::NotFound(format!("Product {} not found", id)))?;
    Ok(Json(product))
}
*/
