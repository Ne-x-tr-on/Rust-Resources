use reqwest::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    food: String,
    drink: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    json: Option<PostData>,
    url: String,
    headers: serde_json::Value,
}

/// Async GET request
pub async fn get_example() -> Result<(), Error> {
    let url = "https://httpbin.org/get";
    let client = Client::new();

    let response = client.get(url).send().await?;
    println!("GET Status: {}", response.status());
    println!("GET Body:\n{}", response.text().await?);

    Ok(())
}

/// Async POST request with JSON
pub async fn post_example() -> Result<(), Error> {
    let url = "https://httpbin.org/post";
    let client = Client::new();

    let new_data = PostData {
        food: "Sandwich".to_string(),
        drink: "Water".to_string(),
    };

    let response = client.post(url).json(&new_data).send().await?;
    println!("POST Status: {}", response.status());

    let parsed: ResponseData = response.json().await?;
    println!("POST Response Parsed: {:#?}", parsed);

    Ok(())
}

/// Async PUT request
pub async fn put_example() -> Result<(), Error> {
    let url = "https://httpbin.org/put";
    let client = Client::new();

    let update_data = PostData {
        food: "Sushi".to_string(),
        drink: "Tea".to_string(),
    };

    let response = client.put(url).json(&update_data).send().await?;
    println!("PUT Status: {}", response.status());

    let parsed: ResponseData = response.json().await?;
    println!("PUT Response Parsed: {:#?}", parsed);

    Ok(())
}

/// Async DELETE request
pub async fn delete_example() -> Result<(), Error> {
    let url = "https://httpbin.org/delete";
    let client = Client::new();

    let response = client.delete(url).send().await?;
    println!("DELETE Status: {}", response.status());
    println!("DELETE Body:\n{}", response.text().await?);

    Ok(())
}

/// Helper to run all async examples
pub async fn run_async_examples() {
    if let Err(e) = get_example().await {
        eprintln!("Error in GET: {:#?}", e);
    }

    if let Err(e) = post_example().await {
        eprintln!("Error in POST: {:#?}", e);
    }

    if let Err(e) = put_example().await {
        eprintln!("Error in PUT: {:#?}", e);
    }

    if let Err(e) = delete_example().await {
        eprintln!("Error in DELETE: {:#?}", e);
    }

  // recap::async_::run_async_examples().await;
}
