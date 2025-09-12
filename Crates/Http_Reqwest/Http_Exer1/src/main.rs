// use reqwest::blocking;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 1. Send a GET request to the website
//     let response = blocking::get("https://httpbin.org/get")?;

//     // 2. Turn the response into text
//     let body = response.text()?;

//     // 3. Print it out
//     println!("Website says: {}", body);

//     Ok(())
// }


// use reqwest::blocking::Client;
// use std::collections::HashMap;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();

//     let mut order = HashMap::new();
//     order.insert("food", "Cheeseburger");
//     order.insert("drink", "Coke");

//     let res = client.post("https://httpbin.org/post")
//         .json(&order)
//         .send()?;

//     println!("Server reply: {}", res.text()?);

//     Ok(())
// }


// use reqwest::blocking::Client;
// use serde::Deserialize;
// use std::collections::HashMap;

// #[derive(Deserialize, Debug)]
// struct ServerReply {
//     json: HashMap<String, String>,
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();

//     let mut order = HashMap::new();
//     order.insert("food".to_string(), "Cheeseburger".to_string());
//     order.insert("drink".to_string(), "Coke".to_string());

//     let res = client
//         .post("https://httpbin.org/post")
//         .json(&order)
//         .send()?
//         .json::<ServerReply>()?; // <-- Deserialize into struct

//     println!("Parsed reply: {:?}", res);

//     // Accessing values
//     println!("Food ordered: {}", res.json["food"]);
//     println!("Drink ordered: {}", res.json["drink"]);

//     Ok(())
// }

use axum::{routing::get, Router};
use std::net::SocketAddr;
use hyper::server::Server; // Server comes via hyper-util under the hood

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum 🚀" }))
        .route("/ping", get(|| async { "Pong 🏓" }));

    // Address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Run server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}





