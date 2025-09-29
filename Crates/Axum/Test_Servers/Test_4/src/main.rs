use axum::{
    response::IntoResponse, routing::get, Router
};

use std::net::SocketAddr;

#[tokio::main]
async fn main(){
let app = Router::new()
    .route("/",get(hello_handler).post(sendhello_handler));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server Running on port:\n{:?}",addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();
}

async fn hello_handler() -> impl IntoResponse{
    println!("Hello Mr Welcome to axum framework")
}

async fn sendhello_handler() -> impl IntoResponse{
    "Hey"
}