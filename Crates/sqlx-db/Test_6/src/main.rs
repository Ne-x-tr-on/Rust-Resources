use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Failed to read from the env file");
    println!("Env handshake Confirmed");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}
