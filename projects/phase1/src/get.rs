use reqwest;
use tokio;
use std::error::Error;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main()-> Result<(),Box<dyn Error>>{
    dotenv().ok();
    let url = env::var("PROFILE").expect("Failed to fetch url");
    let http_request = reqwest::get(&url).await?;
    

    Ok(())
}
