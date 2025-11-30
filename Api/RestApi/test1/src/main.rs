use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug,Serialize)]
struct Message{
    message:String,
}

async fn hello()->Json<Message>{
    Json(Message { message: "David Kamau".to_string(), })
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/hello",get(hello));

   let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
   let port= 3000;
   let socket = SocketAddr::new(ip, port);

   println!("Server Running on https:{}",socket);

   let listener = tokio::net::TcpListener::bind(&socket).await.unwrap();

   axum::serve(listener, app).await.unwrap();

//    axum::serve(socket, app).await.unwrap();
}