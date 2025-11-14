use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::Request;
use axum::response::Response;
use axum::body::BoxBody;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_jwt(user_id: &Uuid, secret: &str, ttl_seconds: usize) -> Result<String, JwtError> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    let claims = Claims { sub: *user_id, exp: start + ttl_seconds };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, JwtError> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()).map(|data| data.claims)
}

// AuthLayer: simple middleware to parse Authorization header and attach claims as Extension
use axum::extract::Extension;
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::sync::Arc;
use sqlx::PgPool;
use axum::http::header::AUTHORIZATION;

pub struct AuthLayer {
    pool: Arc<PgPool>,
}

impl AuthLayer {
    pub fn new() -> impl Fn(Request<axum::body::BoxBody>, Next<axum::body::BoxBody>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::Error>> + Send + 'static>> + Clone {
        move |mut req, next| {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            Box::pin(async move {
                let auth_header = req.headers().get(AUTHORIZATION).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
                if let Some(h) = auth_header {
                    if h.starts_with("Bearer ") {
                        let token = h.trim_start_matches("Bearer ").trim();
                        match crate::auth::verify_jwt(token, &secret) {
                            Ok(claims) => {
                                // attach claims as Extension
                                req.extensions_mut().insert(Some(claims));
                            }
                            Err(_) => { req.extensions_mut().insert(None as Option<crate::auth::Claims>); }
                        }
                    }
                }
                let resp = next.run(req).await;
                Ok(resp)
            })
        }
    }
}