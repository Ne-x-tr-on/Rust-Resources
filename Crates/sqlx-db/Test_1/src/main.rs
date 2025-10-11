use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please enter a valid database url");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await;
     println!("Databse Connection has been Established")
}