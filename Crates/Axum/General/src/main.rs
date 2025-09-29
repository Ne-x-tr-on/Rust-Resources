use axum::{
    routing::get,
    Router,
};

use std::net::SocketAddr;

#[allow(unused)]
#[tokio::main]
async fn main(){
    let tryapp = Router::new()
    .route("/",get(logdata));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server Running on port\n{:?}",addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, tryapp).await.unwrap();
}

async fn logdata(){
    println!("Log data Sent");
}

