
use axum::{Router, routing::{get, post}};
use crate::handlers; // import the re-exported functions

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", get(handlers::get_user).post(handlers::create_user))
}
