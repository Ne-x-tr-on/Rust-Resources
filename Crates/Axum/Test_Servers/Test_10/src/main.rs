use axum::{
    extract::State,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json, Router,
    routing::get,
};
use serde::{Serialize, Deserialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber};

// -----------------------------------------------------------------------------
// Models
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    age: i32,
}

// -----------------------------------------------------------------------------
// Application shared state
// -----------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    app_name: String,
    version: String,
}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("🚀 Starting production server...");

    // Shared state
    let state = Arc::new(AppState {
        app_name: "Newton API".to_string(),
        version: "1.0.0".to_string(),
    });

    // Router
    let app = Router::new()
        .route("/", get(index_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http()); // logging middleware

    // Bind server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("✅ Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// -----------------------------------------------------------------------------
// Handlers
// -----------------------------------------------------------------------------

async fn index_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user = User {
        username: "Newton".to_string(),
        age: 19,
    };

    info!("User accessed: {:?}", user);

    Json(serde_json::json!({
        "status": "success",
        "message": format!("Welcome to {} v{}", state.app_name, state.version),
        "data": user
    }))
}

// -----------------------------------------------------------------------------
// Custom error type (Optional for production)
// -----------------------------------------------------------------------------

#[derive(Debug)]
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": self.0
            })),
        )
            .into_response()
    }
}
