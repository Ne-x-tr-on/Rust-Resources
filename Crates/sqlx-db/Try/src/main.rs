pub mod db;
pub mod models;


use axum::{
    Router,
    routing::get,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
fn main() {
    println!("Hello, world!");
}
