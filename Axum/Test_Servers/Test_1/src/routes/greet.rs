use axum::{
    routing::get,
    Router,
};

async fn greet() -> &'static str {
    "Hello from greet route!"
}

pub fn greet_routes() -> Router {
    Router::new().route("/greet", get(greet))
}
