mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod services;
mod utils;

use axum::{Router, routing::get, Extension};
use dotenvy::dotenv;
use std::env;
use db::init_db;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize DB pool
    let pool = init_db().await?;

    // Build router
    let app = Router::new()
        .nest("/auth", routes::auth::auth_routes())
        .layer(Extension(pool))
        .layer(CookieManagerLayer::new());

    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("Server running on http://0.0.0.0:{}", port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
