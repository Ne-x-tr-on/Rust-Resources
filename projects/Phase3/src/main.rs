use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use axum::http::StatusCode;
use once_cell::sync::Lazy;

#[derive(Clone)]
struct AppState {
    motor_state: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        motor_state: Arc::new(Mutex::new(false)),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/motor/on", post(turn_on))
        .route("/motor/off", post(turn_off))
        .route("/motor/status", get(status))
        .with_state(state.clone());

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// -----------------------------
// Handlers
// -----------------------------

async fn index() -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Conveyor Control</title>
        <style>
            body { font-family: Arial; text-align: center; padding-top: 40px; background: #f4f4f4; }
            button { margin: 20px; padding: 15px 30px; font-size: 18px; border: none; border-radius: 8px; cursor: pointer; }
            .on { background: #4CAF50; color: white; }
            .off { background: #f44336; color: white; }
        </style>
    </head>
    <body>
        <h1>🚦 Conveyor Motor Control</h1>
        <button class="on" onclick="controlMotor('on')">Turn ON</button>
        <button class="off" onclick="controlMotor('off')">Turn OFF</button>
        <p id="status"></p>

        <script>
            async function controlMotor(action) {
                await fetch(`/motor/${action}`, { method: 'POST' });
                checkStatus();
            }

            async function checkStatus() {
                const res = await fetch('/motor/status');
                const text = await res.text();
                document.getElementById('status').innerText = text;
            }

            checkStatus();
            setInterval(checkStatus, 3000);
        </script>
    </body>
    </html>
    "#;
    Html(html.to_string())
}

async fn turn_on(State(state): State<AppState>) -> impl IntoResponse {
    let mut motor = state.motor_state.lock().unwrap();
    *motor = true;
    StatusCode::OK
}

async fn turn_off(State(state): State<AppState>) -> impl IntoResponse {
    let mut motor = state.motor_state.lock().unwrap();
    *motor = false;
    StatusCode::OK
}

async fn status(State(state): State<AppState>) -> impl IntoResponse {
    let motor = state.motor_state.lock().unwrap();
    if *motor {
        "Motor is ON".to_string()
    } else {
        "Motor is OFF".to_string()
    }
}
