use axum::{
    Router,
    routing::get,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

mod routes;
mod db;
mod handlers;
mod models;
mod auth;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Setup DB pool
    let pool = db::init_pool().await?;

    // Build router
    let app = routes::create_router(pool.clone());

    let addr: SocketAddr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()?;

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
