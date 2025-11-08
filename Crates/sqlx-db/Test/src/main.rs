pub mod insert;
pub mod model;
pub mod delete;

use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use tokio;
use std::error::Error;


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
read_db().await?;
    
Ok(())
}


async fn read_db()->Result<(),sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to fetch from the database");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to the Database");

    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to run migrations");
    Ok(())
}