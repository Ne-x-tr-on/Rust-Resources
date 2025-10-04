pub mod users;
use axum::{
    response::IntoResponse, routing::{get,post}, serve::Listener, Json, Router
};

use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/users",get(users::get_user));
   

let addr = SocketAddr::from(([127,0,0,1],3000));
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(listener,app).await.unwrap();
}
