#[allow(unused)]
use reqwest::blocking::{Client,ClientBuilder};
use reqwest::redirect::Policy;

pub fn getmeth(){
  let url = "http://localhost:3000/get_data";

  let http_client = Client::new();

  let response = http_client.get(url).send().unwrap();
  println!("Response:\n{:#?}",response.status());

 let body = response.text().unwrap();
 println!("Data:\n{:#?}",body);

}

pub fn client_builder(){
  //  ClientBuilder 
// Example redirect


let url = "http://localhost:3000/weather";
let redir_policy = Policy::limited(5);
let http_client = ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();
let http_result = http_client.get(url).send();
}


