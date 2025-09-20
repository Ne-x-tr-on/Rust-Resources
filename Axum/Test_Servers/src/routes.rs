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
