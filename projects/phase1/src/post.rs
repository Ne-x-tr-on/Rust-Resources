use std::error::Error;
use reqwest::Client;
use dotenvy::dotenv;
use std::env;
use serde_json::json;

pub async fn post_data()->Result<(),Box<dyn Error>>{
  dotenv().ok();
  let url = env::var("LOCALHOST").expect("Failed to post");

  let client = Client::new();
  let payload = serde_json::json!({
    "name":"Newton",
    "age":"19",
  });

  let response = client
    .post(&url)
    .json(&payload)
    .send()
    .await?;
   
   println!("Status: {:?}",response.status());
   let text = response.text().await?;
   println!("Text:{:?}",text);

  Ok(())
}