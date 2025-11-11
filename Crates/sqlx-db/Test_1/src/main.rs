pub mod insert;
pub mod model;
pub mod select;
// pub mod update;
// pub mod delete;


use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::insert::insert_profile;
use crate::select::select_profile;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to the database!");

    let result =insert_profile(&pool, "Newton".to_string(),"Newton Kamau".to_string(), "23newton@gmail.com".to_string());

    match select_profile(&pool, 1).await? {
        Some(profile) => println!("result {:?}", profile),
        None => println!("Profile with ID 1 not found"),
    }

    let result =insert_profile(&pool, "Newton".to_string(),"Newton Kamau".to_string(), "23newton@gmail.com".to_string());

    Ok(())
}
