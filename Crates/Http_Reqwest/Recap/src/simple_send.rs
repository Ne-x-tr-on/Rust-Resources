use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error as StdError; // rename the trait to avoid conflict

pub async fn post_example() -> Result<(), Box<dyn StdError>> {
    let url = "https://httpbin.org/post";
    let client = Client::new();

    // Data to send
    let new_data = json!({
        "food": "Sandwich",
        "drink": "Water"
    });

    // Send POST request
    let response = client.post(url)
        .json(&new_data)
        .send()
        .await?;

    // Status
    println!("Post Status: {}", response.status());

    // Parse response JSON
    let parsed: Value = response.json().await?;
    println!("Parsed JSON:\n{:#?}", parsed);

    Ok(())
}
