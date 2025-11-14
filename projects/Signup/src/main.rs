use axum::{
    routing::{get, post},
    extract::{Path, State, Query},
    Json, Router, http::StatusCode,
    middleware,
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use sqlx::PgPool;
use tracing::{info, error};

mod models;
mod auth;
mod errors;

use models::{UserInput, SignupResponse, LoginInput};
use auth::{create_jwt, AuthLayer};

type SharedPool = Arc<PgPool>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    let shared_pool = Arc::new(pool);

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/verify_email", get(verify_email))
        .route("/login", post(login))
        .route("/resend_verification", post(resend_verification))
        // Protected example
        .route("/me", get(me))
        .layer(middleware::from_fn_with_state(shared_pool.clone(), AuthLayer::new()))
        .with_state(shared_pool);

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3000".into()).parse()?;
    let addr = SocketAddr::from(([0,0,0,0], port));
    info!(%addr, "Starting server");

    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

// ---------------- HANDLERS ----------------

async fn signup(State(pool): State<SharedPool>, Json(payload): Json<UserInput>) -> Result<(StatusCode, Json<SignupResponse>), (StatusCode, String)> {
    match models::create_user(&pool, payload).await {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => {
            error!("signup error: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

async fn verify_email(Query(params): Query<std::collections::HashMap<String, String>>, State(pool): State<SharedPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let token = match params.get("token") {
        Some(t) => t.clone(),
        None => return Err((StatusCode::BAD_REQUEST, "token missing".into())),
    };
    match models::verify_email(&pool, &token).await {
        Ok(_) => Ok((StatusCode::OK, "Email verified".into())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn login(State(pool): State<SharedPool>, Json(payload): Json<LoginInput>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match models::login_user(&pool, payload).await {
        Ok(user) => {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let token = create_jwt(&user.id, &secret, 60 * 60)?; // 1 hour
            Ok(Json(serde_json::json!({"token": token, "user": {"id": user.id, "email": user.email}})))
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
    }
}

async fn resend_verification(State(pool): State<SharedPool>, Json(payload): Json<std::collections::HashMap<String,String>>) -> Result<StatusCode, (StatusCode, String)> {
    let email = payload.get("email").cloned().ok_or((StatusCode::BAD_REQUEST, "email missing".into()))?;
    match models::resend_verification(&pool, &email).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn me(State(pool): State<SharedPool>, axum::extract::Extension(claims): axum::extract::Extension<Option<auth::Claims>>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(claims) = claims {
        match models::get_user_by_id(&pool, &claims.sub).await {
            Ok(u) => Ok(Json(serde_json::json!({"id": u.id, "email": u.email, "is_verified": u.is_verified}))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, "missing token".into()))
    }
}








