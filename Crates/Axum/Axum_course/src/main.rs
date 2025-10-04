pub mod vehicle;
pub mod login;
pub mod login_DB;
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
// use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/vehicle", get(vehicle::vehicle_get).post(vehicle::vehicle_post))
        .route("/bank_form", get(bank_form));
        // .route("/login", post(login)); 

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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


