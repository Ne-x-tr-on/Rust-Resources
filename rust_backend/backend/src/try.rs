use axum::{
  routing::get,
  response::html,
  Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main(){
  println!("Backend is starting");

  let routespipe = Router::new().route(
      get(|| async {Html("Pipe running succesfully")}),
  );


}