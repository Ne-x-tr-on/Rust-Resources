use axum::{
    routing::{get, post},
    Router,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::model::{LoginInfo, LoginResponse, Claims};
use crate::middleware::AuthUser;

mod middleware;
mod model;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/protected", get(protected).layer(axum::middleware::from_extractor::<AuthUser>()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "JWT auth example running!"
}

async fn login(Json(body): Json<LoginInfo>) -> Json<LoginResponse> {
    let expiration = (Utc::now() + Duration::hours(1)).timestamp() as usize;

    let claims = Claims {
        sub: body.username,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("MY_SECRET".as_ref()),
    )
    .unwrap();

    Json(LoginResponse { token })
}

async fn protected(AuthUser: AuthUser) -> &'static str {
    "This is a PROTECTED route — JWT valid!"
}
