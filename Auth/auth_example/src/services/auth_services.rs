use sqlx::PgPool;
use crate::utils::password::{hash_password, verify_password};
use crate::utils::jwt::{create_access_token, create_refresh_token, decode_token, Claims};
use uuid::Uuid;
use argon2::{Argon2, password_hash::PasswordHash, PasswordVerifier};

pub async fn signup(pool: &PgPool, username: &str, password: &str) -> anyhow::Result<()> {
    let hashed = hash_password(password)?;
    sqlx::query!("INSERT INTO users (username, password_hash) VALUES ($1, $2)", username, hashed)
        .execute(pool).await?;
    Ok(())
}

pub async fn login(pool: &PgPool, username: &str, password: &str) -> anyhow::Result<(String,String)> {
    let user = sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", username)
        .fetch_one(pool).await?;

    if !verify_password(&user.password_hash, password) {
        anyhow::bail!("invalid credentials");
    }

    let access = create_access_token(&user.id.to_string())?;
    let jti = Uuid::new_v4().to_string();
    let refresh = create_refresh_token(&user.id.to_string(), &jti)?;

    // store hashed jti
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(jti.as_bytes(), &argon2::password_hash::SaltString::generate(&mut rand::rngs::OsRng))?.to_string();
    sqlx::query!("INSERT INTO refresh_tokens (user_id, jti_hash, expires_at, revoked) VALUES ($1,$2,now() + interval '7 days', false)", user.id, password_hash)
        .execute(pool).await?;

    Ok((access, refresh))
}

// Refresh token verification + rotation omitted for brevity (same as previous example)
