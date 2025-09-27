use axum::{
    routing::get,
    Router,
};

async fn hello() -> &'static str {
    "Hello 👋 Here is my first Route"
}

pub fn main_router() -> Router {
    Router::new()
        .route("/hello", get(hello))
}


// Cargo.toml must include axum, tokio, serde, serde_json, reqwest, sqlx (optional)
use axum::{
    routing::{get, post},
    Router, Json, extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Shared app state
#[derive(Clone)]
struct AppState {
    // e.g., database pool or reqwest client
    client: reqwest::Client,
    // db_pool: PgPool, etc.
}

#[derive(Deserialize)]
struct AskRequest {
    question: String,
}

#[derive(Serialize)]
struct AskResponse {
    answer: String,
}

// Handler for GET /
async fn root() -> &'static str {
    "AI Tutors backend running"
}

// Handler that routes AI requests by subject
async fn ask_ai(
    State(state): State<AppState>,
    Path(subject): Path<String>,          // e.g., "maths" or "english"
    Json(payload): Json<AskRequest>,
) -> Result<Json<AskResponse>, (StatusCode, String)> {
    // Choose python microservice port by subject (example)
    let url = match subject.as_str() {
        "maths" => "http://maths_ai:8001/predict",
        "english" => "http://english_ai:8002/predict",
        _ => return Err((StatusCode::BAD_REQUEST, "unknown subject".into())),
    };

    // Forward request to Python AI microservice
    let res = state.client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("upstream error: {}", e)))?;

    if !res.status().is_success() {
        return Err((StatusCode::BAD_GATEWAY, format!("upstream bad status: {}", res.status())));
    }

    let json: serde_json::Value = res.json().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("invalid upstream json: {}", e)))?;

    // Example: assume upstream returns { "answer": "..." }
    let answer = json.get("answer")
        .and_then(|v| v.as_str())
        .unwrap_or("no answer")
        .to_string();

    Ok(Json(AskResponse { answer }))
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        client: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/", get(root))
        // dynamic route: POST /ai/:subject
        .route("/ai/:subject", post(ask_ai))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
