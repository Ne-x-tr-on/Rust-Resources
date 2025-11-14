use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Validation error: {0}")]
    Validation(String),
}