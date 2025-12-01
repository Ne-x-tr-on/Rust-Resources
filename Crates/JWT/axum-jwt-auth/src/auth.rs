use axum::{Json, http::StatusCode};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use crate::model::{LoginInfo, LoginResponse, Claims};

/// Secret key to sign the token
/// (In production: put inside .env)
const SECRET: &[u8] = b"super_secret_key";

// Fake user validation
pub fn is_valid_user(username: &str, password: &str) -> bool {
    username == "newton" && password == "1234"
}

/// POST /login
pub async fn login_handler(
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<LoginResponse>, StatusCode> {

    if !is_valid_user(&login_info.username, &login_info.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create JWT claims
    let claims = Claims {
        sub: login_info.username.clone(),
        exp: (Utc::now() + Duration::minutes(30)).timestamp() as usize,
    };

    // Encode token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}

/// Verifies a JWT and returns the claims
pub fn verify_token(token: &str) -> Result<Claims, StatusCode> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
