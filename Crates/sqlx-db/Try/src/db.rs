use sqlx::{postgres::PgPoolOptions,PgPool};
// use serde::{Serialize,Deserialize};
use dotenvy::dotenv;
use std::env;

pub async fn create_connection() -> Result<PgPool,sqlx::Error>{
  dotenv().ok();

  let db_url = env::var("DATABASE_URL").expect("Failed to read the database");

  // Connection Pool
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    .expect("Failed to connect to the database");

  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to run db migrations");


  Ok(pool)

}