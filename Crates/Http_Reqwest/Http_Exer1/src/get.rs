#[allow(unused)]
use reqwest::blocking::{Client,ClientBuilder};

pub fn test_http(){
  let url = "https://nextronspace.netlify.app";

  let http_client = Client::new();

  let http_response = http_client.get(url).send();

  match http_response{
    Ok(response) => {
      println!("Status:\n{}",response.status());

        match response.text(){
          Ok(body) => {
            println!("Body: {:#?}\n",body)
          }
          Err(e) => {
            eprintln!("Error Occured:\n {}",e)
          }
        }
    },
    Err(e) => {
      println!("Error:\n {:#?}",e)
    }
  }
  
}