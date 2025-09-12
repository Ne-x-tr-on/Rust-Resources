pub mod get;
pub mod post;

fn main(){
    // get::test_http();
    post::test_http();
}

// use reqwest;
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Make an HTTP GET request
//     let http_result = reqwest::get("https://httpbin.org/status/200").await;

//     // Process the result
//     match http_result {
//         Ok(response) => {
//             // The .status() method returns the HTTP status code
//             let status = response.status();
//             println!("Status: {} ({})", status, status.as_u16());
            
//             // You can also check specific properties of the status
//             println!("Is success: {}", status.is_success());
//             println!("Is client error: {}", status.is_client_error());
//             println!("Is server error: {}", status.is_server_error());
            
//             // If you want to print the body:
//             match response.text().await {
//                 Ok(body) => println!("Body:\n{}", body),
//                 Err(e) => eprintln!("Error reading body: {}", e),
//             }
//         }
//         Err(e) => {
//             eprintln!("Request failed: {}", e);
//         }
//     }

//     Ok(())
// }


// use reqwest::blocking::Client;

// fn main() {
//     // The URL must include the scheme (http/https)
//     let url = "https://nextronspace.netlify.app";

//     // Create the HTTP client
//     let http_client = Client::new();

//     // Send GET request
//     let http_result = http_client.get(url).send();

//     match http_result {
//         Ok(response) => {
//             println!("Status: {}", response.status());
//             // If you want to print the body:
//             match response.text() {
//                 Ok(body) => println!("Body:\n{}", body),
//                 Err(e) => eprintln!("Error reading body: {}", e),
//             }
//         }
//         Err(e) => {
//             eprintln!("Request failed: {}", e);
//         }
//     }
// }


