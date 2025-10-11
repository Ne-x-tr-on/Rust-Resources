use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main()->Result<(),sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to connect with the Database");

    let pool:PgPool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await?;
    Ok(())
}