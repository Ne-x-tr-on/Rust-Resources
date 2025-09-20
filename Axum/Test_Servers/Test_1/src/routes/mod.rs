pub mod greet;

use axum::Router;

pub fn main_router() -> Router {
    Router::new()
        .merge(greet::greet_routes())
}
