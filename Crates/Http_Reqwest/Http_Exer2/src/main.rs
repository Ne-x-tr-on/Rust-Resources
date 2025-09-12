// pub mod basic_client;

// fn main() {
//     println!("Hello, world!");
// }

// use reqwest;
// use tokio;

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let response = reqwest::get("https://httpbin.org/get")
//         .await?
//         .text()
//         .await?;

//     println!("Response: {}", response);
//     Ok(())
// }


// use reqwest::Client;
// use serde::Serialize;
// use tokio;

// #[derive(Serialize)]
// struct User {
//     username: String,
//     email: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let client = Client::new();

//     let new_user = User {
//         username: "Newton".to_string(),
//         email: "newton@example.com".to_string(),
//     };

//     let response = client.post("https://httpbin.org/post")
//         .json(&new_user)
//         .send()
//         .await?
//         .text()
//         .await?;

//     println!("Response: {}", response);
//     Ok(())
// }


// use reqwest; // HTTP client
// use tokio;   // Needed because reqwest is async

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a client
//     let client = reqwest::Client::new();

//     // Send a GET request to a webpage
//     let response = client
//         .get("https://www.rust-lang.org")
//         .send()
//         .await?;

//     // Get the response body as text
//     let body = response.text().await?;

//     // Print the first 500 characters of the webpage
//     println!("Webpage content (first 500 chars):\n{}", &body[..500.min(body.len())]);

//     Ok(())
// }

use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client
    let client = reqwest::Client::new();

    // Send GET request to the JSON API
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    // Parse the response body as JSON into our struct
    let post: Post = response.json().await?;

    // Print the deserialized struct
    println!("Post received: {:?}", post);

    Ok(())
}


