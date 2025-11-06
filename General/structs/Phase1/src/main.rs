  
use axum::{
    extract::Json,
    routing::post,
    Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use uuid::Uuid;
use std::error::Error;

// Shared state structure
#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u32>>,
}

// User struct for JSON
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Uuid,
    username: String,
    email: String,
}

// POST handler: receives JSON and increments counter
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<User>
)->Json<User>{
    let mut count = state.counter.lock().unwrap();
    *count+= 1;

    println!("User count: {:?}",count);
    
    Json(payload)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize shared counter
    let state = AppState {
        counter: Arc::new(Mutex::new(10)),
    };

let app = Router::new().route("/",post(create_user)
.with_state(state));


    let addr = SocketAddr::from(([127,0,0,1],3000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server Running on https://{:?}",addr);

    axum::serve(listener,app).await.unwrap();

    Ok(())
}
