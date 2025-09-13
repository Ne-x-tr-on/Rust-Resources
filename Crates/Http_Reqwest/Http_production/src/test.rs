use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let mut data = HashMap::new();
    data.insert("name", "Newton");

    let response = client.post("https://httpbin.org/post")
        .json(&data)
        .send()
        .await?
        .text()
        .await?;

    println!("Server replied: {}", response);
    Ok(())
}
