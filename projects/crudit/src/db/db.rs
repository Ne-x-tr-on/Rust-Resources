use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

pub async fn create_pool()-> sqlx::Pool<sqlx::Postgres>{
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("Failed to connect to the database");
  let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(&database_url)
      .await
      .expect("Failed to create pool");
  pool
}