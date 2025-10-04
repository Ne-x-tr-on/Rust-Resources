use axum::{response::IntoResponse, Json, extract::State};
use sqlx::PgPool;
use argon2::{self, PasswordHash, PasswordVerifier};
use serde_json::json;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}


pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Fetch user from DB
    let result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(user)) => {
            // ✅ Verify password hash
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            let is_valid = argon2::Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .is_ok();

            if is_valid {
                let response = LoginResponse {
                    status: "success".to_string(),
                    message: format!("Welcome back, {}!", user.username),
                };
                Json(response)
            } else {
                let response = LoginResponse {
                    status: "error".to_string(),
                    message: "Invalid credentials".to_string(),
                };
                Json(response)
            }
        }
        Ok(None) => {
            let response = LoginResponse {
                status: "error".to_string(),
                message: "User not found".to_string(),
            };
            Json(response)
        }
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            });
            Json(response)
        }
    }
}
