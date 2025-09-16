use reqwest::Client;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    food: String,
    drink: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ResponseData {
//     json: Option<PostData>,
//     headers: serde_json::Value,
// }

pub async fn get_example() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:3000/get";
    let client = Client::new();

    // Send GET request
    let response = client.get(url).send().await?;

    // Print HTTP status
    println!("Status: {}", response.status());

    // Parse JSON response into serde_json::Value
    let body: serde_json::Value = response.json().await?;
    println!("Body:\n{:#?}", body);

    Ok(())
}


pub async fn post_example() -> Result<(),Box<dyn Error>>{
  let url = "http://localhost:3000/post";
  let client = Client::new();

  // let new_data = PostData{
  //   food:"SandWitch".to_string(),
  //   drink:"Water".to_string(),
  // };

  let new_data = json!({"food":"sandwitch"});
  let response = client.post(url).json(&new_data).send().await?;

  
  // let response = client.post(url).json(&new_data).send().await?;
  let response_status = response.status();
  println!("Post Status:\n{:#?}",response_status);

  // clonning with bytes
  let bytes = response.bytes().await?;

  let response_body = String::from_utf8_lossy(&bytes);
  println!("Response:\n{:#?}",response_body);

  let Parsed:serde_json::Value= serde_json::from_slice(&bytes)?;
  println!("Serde json values: \n{:#?}",Parsed);

  

  Ok(())
}