use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json,
    extract::Json as ExtractJson, 
};
use serde::{
    Serialize,
     Deserialize
}; 

use std::net::SocketAddr;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/vehicle", get(vehicle_get).post(vehicle_post))
        .route("/bank_form", get(bank_form))
        .route("/login", post(login)); // ✅ separated login route

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

async fn vehicle_get() -> impl IntoResponse {
    let vehicle = Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "Corolla".to_string(),
        year: 2020,
        id: Uuid::new_v4().to_string(),
    };
    Json(vehicle)
}

async fn vehicle_post() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "success",
        "message": "Vehicle POST request received"
    });
    Json(response)
}

#[derive(Serialize)]
struct Form {
    name: String,
    gender: String,
    phone_no: String, // ✅ fixed to String instead of i32
}

async fn bank_form() -> impl IntoResponse {
    let test_form = Form {
        name: "Benjamin".to_string(),
        gender: "Male".to_string(),
        phone_no: "0701090807".to_string(),
    };
    Json(test_form)
}

#[derive(Deserialize)] // ✅ to accept JSON from client
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    status: String,
    message: String,
}

async fn login(ExtractJson(payload): ExtractJson<LoginRequest>) -> impl IntoResponse {
    // ✅ Normally you'd verify the password hash from DB
    if payload.username == "Newton" && payload.password == "password123" {
        let response = LoginResponse {
            status: "success".to_string(),
            message: "Login successful!".to_string(),
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
