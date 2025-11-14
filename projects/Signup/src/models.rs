
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use thiserror::Error;
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, PasswordHash, PasswordHasher as PH}};
use rand_core::OsRng;
use lettre::message::Mailbox;

#[derive(Debug, Deserialize)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Error)]
#[error("Model error: {0}")]
pub struct ModelError(String);

impl From<anyhow::Error> for ModelError {
    fn from(e: anyhow::Error) -> Self { ModelError(e.to_string()) }
}

pub async fn create_user(pool: &PgPool, input: UserInput) -> Result<SignupResponse, ModelError> {
    // check existing
    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE email=$1")
        .bind(&input.email)
        .fetch_optional(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;
    if existing.is_some() {
        return Err(ModelError("Email already exists".into()));
    }

    // hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(input.password.as_bytes(), &salt)
        .map_err(|e| ModelError(e.to_string()))?.to_string();

    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query("INSERT INTO users (id, email, password_hash, is_verified, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(id)
        .bind(&input.email)
        .bind(&password_hash)
        .bind(false)
        .bind(now)
        .execute(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;

    // create verification token
    let token = Uuid::new_v4().to_string();
    let expires = Utc::now() + chrono::Duration::hours(24);
    sqlx::query("INSERT INTO email_verifications (user_id, token, expires_at) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(&token)
        .bind(expires)
        .execute(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;

    // send email (async)
    let _ = send_verification_email(&input.email, &token).await;

    Ok(SignupResponse { id, email: input.email })
}

pub async fn verify_email(pool: &PgPool, token: &str) -> Result<(), ModelError> {
    let rec: Option<(Uuid, chrono::DateTime<Utc>)> = sqlx::query_as("SELECT user_id, expires_at FROM email_verifications WHERE token=$1")
        .bind(token)
        .fetch_optional(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;
    let (user_id, expires) = rec.ok_or(ModelError("Invalid token".into()))?;
    if Utc::now() > expires {
        return Err(ModelError("Token expired".into()));
    }
    sqlx::query("UPDATE users SET is_verified=true WHERE id=$1").bind(user_id).execute(pool).await.map_err(|e| ModelError(e.to_string()))?;
    sqlx::query("DELETE FROM email_verifications WHERE token=$1").bind(token).execute(pool).await.map_err(|e| ModelError(e.to_string()))?;
    Ok(())
}

pub async fn resend_verification(pool: &PgPool, email: &str) -> Result<(), ModelError> {
    let user: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE email=$1 AND is_verified=false").bind(email).fetch_optional(pool).await.map_err(|e| ModelError(e.to_string()))?;
    let (user_id,) = user.ok_or(ModelError("No unverified user with that email".into()))?;
    let token = Uuid::new_v4().to_string();
    let expires = Utc::now() + chrono::Duration::hours(24);
    sqlx::query("INSERT INTO email_verifications (user_id, token, expires_at) VALUES ($1, $2, $3)")
        .bind(user_id).bind(&token).bind(expires).execute(pool).await.map_err(|e| ModelError(e.to_string()))?;
    let _ = send_verification_email(email, &token).await;
    Ok(())
}

pub async fn login_user(pool: &PgPool, input: LoginInput) -> Result<User, ModelError> {
    let rec: Option<(Uuid, String, bool)> = sqlx::query_as("SELECT id, password_hash, is_verified FROM users WHERE email=$1")
        .bind(&input.email)
        .fetch_optional(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;
    let (id, password_hash, _is_verified) = rec.ok_or(ModelError("Invalid credentials".into()))?;

    let parsed = PasswordHash::new(&password_hash).map_err(|e| ModelError(e.to_string()))?;
    Argon2::default().verify_password(input.password.as_bytes(), &parsed).map_err(|_| ModelError("Invalid credentials".into()))?;

    let user = get_user_by_id(pool, &id).await?;
    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: &uuid::Uuid) -> Result<User, ModelError> {
    let rec: Option<(Uuid, String, String, bool, chrono::DateTime<Utc>)> = sqlx::query_as("SELECT id, email, password_hash, is_verified, created_at FROM users WHERE id=$1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ModelError(e.to_string()))?;
    let (id, email, password_hash, is_verified, created_at) = rec.ok_or(ModelError("User not found".into()))?;
    Ok(User { id, email, password_hash, is_verified, created_at })
}

async fn send_verification_email(email: &str, token: &str) -> Result<(), ModelError> {
    // Configure SMTP from env
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.example.com".into());
    let smtp_user = std::env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = std::env::var("SMTP_PASS").unwrap_or_default();
    let from = std::env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@example.com".into());

    // Construct verification link (PUBLIC_URL env)
    let public = std::env::var("PUBLIC_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let link = format!("{}/verify_email?token={}", public, token);

    let email_body = format!("Please verify your email by clicking: {}", link);

    let email = lettre::Message::builder()
        .from(from.parse().unwrap_or_else(|_| Mailbox::new(None, "no-reply@example.com".parse().unwrap())))
        .to(email.parse().unwrap())
        .subject("Verify your email")
        .body(email_body)
        .map_err(|e| ModelError(e.to_string()))?;

    let creds = lettre::transport::smtp::authentication::Credentials::new(smtp_user, smtp_pass);
    let mailer = lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&smtp_host)
        .map_err(|e| ModelError(e.to_string()))?
        .credentials(creds)
        .build();

    mailer.send(email).await.map_err(|e| ModelError(e.to_string()))?;
    Ok(())
}

