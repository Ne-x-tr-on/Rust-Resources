use reqwest::blocking::Client;
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

/// Basic GET request
pub fn get_example() -> Result<(), Error> {
    let url = "https://httpbin.org/get";
    let client = Client::new();

    let response = client.get(url).send()?; // ? propagates error
    println!("GET Status: {}", response.status());
    println!("GET Body:\n{}", response.text()?);

    Ok(())
}

/// POST with JSON body
pub fn post_example() -> Result<(), Error> {
    let url = "https://httpbin.org/post";
    let client = Client::new();

    let new_data = PostData {
        food: "Pizza".to_string(),
        drink: "Coke".to_string(),
    };

    let response = client.post(url).json(&new_data).send()?;
    println!("POST Status: {}", response.status());

    let parsed: ResponseData = response.json()?;
    println!("POST Response Parsed: {:#?}", parsed);

    Ok(())
}

/// PUT for update
pub fn put_example() -> Result<(), Error> {
    let url = "https://httpbin.org/put";
    let client = Client::new();

    let update_data = PostData {
        food: "Burger".to_string(),
        drink: "Juice".to_string(),
    };

    let response = client.put(url).json(&update_data).send()?;
    println!("PUT Status: {}", response.status());

    let parsed: ResponseData = response.json()?;
    println!("PUT Response Parsed: {:#?}", parsed);

    Ok(())
}

/// DELETE request
pub fn delete_example() -> Result<(), Error> {
    let url = "https://httpbin.org/delete";
    let client = Client::new();

    let response = client.delete(url).send()?;
    println!("DELETE Status: {}", response.status());
    println!("DELETE Body:\n{}", response.text()?);

    Ok(())
}
