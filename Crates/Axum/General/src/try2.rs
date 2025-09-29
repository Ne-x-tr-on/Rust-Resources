use axum::{
  routing::get,
  Router,
};

use std::net::SocketAddr;

#[allow(unused)]
#[tokio::main]
async fn main(){
  let myapp = Router::new()
    .route("/",get(myname));
  let addr = SocketAddr::from(([127,0,0,1],3000));
  println!("Server running on Port:\n{:?}",addr);

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener,app).await.unwrap();

  axum
}

async fn myname(){
  "Hello from Axum"
}