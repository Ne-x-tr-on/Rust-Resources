use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    food: String,
    drink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    json: Option<PostData>,
    url: String,
    headers: serde_json::Value,
}

pub async fn get_example() -> Result<(), Error> {
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


pub async fn post_example() -> Result<(),Error>{
  let url = "http://localhost:3000/post";
  let client = Client::new();

  let new_data = PostData{
    food:"SandWitch".to_string(),
    drink:"Water".to_string(),
  };

  let response = client.post(url).json(&new_data).send().await?;
  println!("Post Status:\n{:#?}",response.status());

  let parsed:ResponseData= response.json().await?;

  Ok(())
}