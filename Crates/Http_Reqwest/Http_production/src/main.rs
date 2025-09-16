// pub mod http;
// pub mod test;


use reqwest::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let url = "https://httpbin.org/status/500"; // will return server error

    let mut attempts = 0;
    let max_retries = 3;

    while attempts < max_retries {
        let response = client.get(url).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                println!("Success: {}", resp.text().await?);
                break;
            }
            Ok(resp) => {
                println!("Server responded with status: {}", resp.status());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        attempts += 1;
        println!("Retrying... attempt {}/{}", attempts, max_retries);
        sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}


