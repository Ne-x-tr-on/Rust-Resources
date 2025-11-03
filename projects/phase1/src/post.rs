use std::error::Error;
use reqwest::Client;
use dotenvy::dotenv;
use std::env;
use serde_json::json;

pub async fn post_data() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Get the URL from .env
    let url = env::var("LOCALHOST").expect("Failed to fetch the URL");

    // Create the HTTP client
    let client = Client::new();

    // The JSON payload
    let payload = json!({
        "name": "Newton",
        "Age": "19"
    });

    // Send the POST request
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;

    // Print the response
    let text = response.text().await?;
    println!("✅ Server Response: {}", text);

    Ok(())
}
