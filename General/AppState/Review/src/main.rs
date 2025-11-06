use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

struct AppState {
    counter: Mutex<u32>,
}

async fn count(State(state): State<Arc<AppState>>) -> String {
    let mut num = state.counter.lock().unwrap(); // lock mutex to safely access
    *num += 1; // increment counter
    format!("Counter: {}", *num)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        counter: Mutex::new(0),
    });

    let app = Router::new()
        .route("/hello/:name", get(greet))
        .route("/count", get(count))
        .with_state(state);

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Server running at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
