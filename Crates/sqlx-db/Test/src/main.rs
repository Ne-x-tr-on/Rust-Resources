pub mod insert;
pub mod model;
pub mod delete;

use sqlx::{PgPool,FromRow,postgres::PgPoolOptions};
use uuid::Uuid;
use chrono::{DateTime,Utc};
use dotenvy::dotenv;
use std::env;
use tokio;
use std::error::Error;
use crate::insert::insert_dev;


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
read_db().await?;

    
Ok(())
}


pub async fn read_db()->Result<(),sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to fetch from the database");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
  

    sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;

    
    let dev = insert_dev(&pool, Uuid::new_v4(), "Newton", Utc::now()).await?;

    Ok(())
}