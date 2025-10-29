use axum::{routing::{get, post}, serve::Listener, Json, Router};
use serde_json::Value;
use std::net::SocketAddr;




async fn receive_data(Json(payload):Json<Value>)->String{
    format!("Received:{:?}",payload)
    

}

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/",post(receive_data));

    let addr = SocketAddr::from(([127,0,0,1],3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();
}