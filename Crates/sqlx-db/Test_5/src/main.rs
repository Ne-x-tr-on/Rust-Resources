use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main(){
read_db().await;
}

async fn read_db(){
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("Failed to read the Url Ensure its the right one");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to Connect to the Database");

sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to read from the Database");

}