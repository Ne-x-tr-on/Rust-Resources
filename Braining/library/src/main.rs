use std::net::SocketAddr;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
mod routes;
mod models;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Could not read the database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = routes::app_router(pool);
    let addr = SocketAddr::from(([127,0,0,0],3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
