// use axum::{
//     routing::post,
//     Router,
//     Json,
// };
// use serde::{Deserialize, Serialize};
// use std::net::SocketAddr;

// #[derive(Deserialize, Serialize)]
// struct SignupData {
//     username: String,
//     password: String,
// }

// #[derive(Serialize)]
// struct ResponseMsg {
//     message: String,
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/signup", post(handle_signup));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Server running on {}", addr);

//     axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
//         .await
//         .unwrap();
// }

// async fn handle_signup(Json(payload): Json<SignupData>) -> Json<ResponseMsg> {
//     println!("Received signup: {} - {}", payload.username, payload.password);

//     // Example: Forward the request with reqwest (like calling another API)
//     let client = reqwest::Client::new();
//     let _res = client.post("https://httpbin.org/post")
//         .json(&payload)
//         .send()
//         .await;

//     // Just respond back
//     Json(ResponseMsg {
//         message: format!("User '{}' signed up successfully!", payload.username),
//     })
// }



use axum::{
    routing::{post, get},
    Router,
    Json,
    response::Html,
    extract::State,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::net::SocketAddr;

#[derive(Deserialize, Serialize)]
struct SignupData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct ResponseMsg {
    message: String,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    // Connect to Postgres
    let db_url = "postgres://postgres:password@localhost:5432/axum_demo";
    let pool = PgPool::connect(db_url).await.expect("❌ Failed to connect to Postgres");

    let state = AppState { db: pool };

    // Routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/signup", post(handle_signup))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn handle_signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupData>,
) -> Json<ResponseMsg> {
    println!("📥 Received signup: {} - {}", payload.username, payload.password);

    // Insert into database
    let query = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        payload.username,
        payload.password
    )
    .execute(&state.db)
    .await;

    match query {
        Ok(_) => Json(ResponseMsg {
            message: format!("✅ User '{}' signed up successfully!", payload.username),
        }),
        Err(err) => Json(ResponseMsg {
            message: format!("❌ Failed to sign up: {}", err),
        }),
    }
}


