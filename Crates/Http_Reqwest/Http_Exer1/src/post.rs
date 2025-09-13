#[allow(unused)]
use reqwest::blocking::Client;
use serde_json::json;

pub fn test_exer1() {
    let http_client = Client::new();
    let url = "http://localhost:3000/send_data";


    // GET request
    // let http_response = http_client.get(url).send();
    // match http_response {
    //     Ok(resp) => println!("✅ GET Status: {}", resp.status()),
    //     Err(e) => eprintln!("❌ GET request failed: {:?}", e),
    // }

    // JSON payload for POST
  let payload = json!({
    "first_name":"Newton",
    "age": 19
  });

  // let payload2 = json!({});

    // POST request
  let post_result = http_client
      .post(url)
      .json(&payload)
      .send();

  match post_result{
    Ok(response)=>{
      println!("Response:{}",response.status())
    }
    Err(e) => {
      eprintln!("Error: {}",e)
    }
  }
}
