use axum::{
  routing::get,
  Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main(){
  let Opticlass = Router::new("/",get(|| async {
    "Hello World"
  }));

let addr = SocketAddr::from((["127.0.0.1"],3000));

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(Opticlass,listener).await.unwrap();

}