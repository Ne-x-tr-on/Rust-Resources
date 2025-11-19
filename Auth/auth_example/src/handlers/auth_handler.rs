use axum::{Extension, Json};
use tower_cookies::Cookies;
use serde::Deserialize;
use sqlx::PgPool;
use crate::services::auth_service;

#[derive(Deserialize)]
pub struct AuthRequest { pub username: String, pub password: String }

pub async fn signup_handler(Extension(pool): Extension<PgPool>, Json(payload): Json<AuthRequest>) -> Json<&'static str> {
    auth_service::signup(&pool, &payload.username, &payload.password).await.unwrap();
    Json("Signup successful")
}

pub async fn login_handler(Extension(pool): Extension<PgPool>, Json(payload): Json<AuthRequest>, cookies: Cookies) -> Json<serde_json::Value> {
    let (access, refresh) = auth_service::login(&pool, &payload.username, &payload.password).await.unwrap();
    cookies.add(tower_cookies::Cookie::new("refresh_token", refresh));
    Json(serde_json::json!({"access_token": access}))
}

pub async fn refresh_handler() -> Json<&'static str> {
    Json("Refresh token endpoint")
}

pub async fn logout_handler() -> Json<&'static str> {
    Json("Logout endpoint")
}
