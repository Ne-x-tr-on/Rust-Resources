use axum::{
    Router,
    routing::{get, post},
    extract::Extension,
};
use sqlx::PgPool;

use crate::handlers;

pub fn create_router(pool: PgPool) -> Router {
    let api = Router::new()
        .route("/", get(|| async { "Opticlass API" }))
        // auth
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        // students
        .route("/api/v1/students", post(handlers::students::create_student))
        .route("/api/v1/students/:id", get(handlers::students::get_student))
        .route("/api/v1/students", get(handlers::students::list_students));

    Router::new()
        .nest("/api", api)
        .layer(Extension(pool))
}
