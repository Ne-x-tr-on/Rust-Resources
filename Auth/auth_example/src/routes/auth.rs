use axum::routing::post;
use axum::Router;
use crate::handlers::auth_handler::{login_handler, signup_handler, refresh_handler, logout_handler};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/logout", post(logout_handler))
}
