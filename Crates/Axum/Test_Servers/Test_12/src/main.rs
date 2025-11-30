use axum::{
    http::{response, StatusCode}, routing::{get,post}, Json, Router
};
use std::error::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Utc};
use uuid::Uuid;
use tokio;
use std::net::SocketAddr;

pub async fn insert_handler(Json(payload):Json<InsertData>)->(StatusCode,Json<InsertResponse>){
    let response = InsertResponse{
        id:Uuid::new_v4(),
        name:payload.name.clone(),
        created_at:Utc::now(),
    };
    println!("User name{:?} has been inserted",payload.name.clone());
    (StatusCode::OK,Json(response))

}



// #[axum::debug_handler]
#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    let app = Router::new()
    .route("/hello",post(insert_handler));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server Running on {}",addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
Ok(())
}
#[derive(Debug,Deserialize)]
struct InsertData{
    name:String,
}
#[derive(Serialize,Debug)]
struct InsertResponse{
    id:Uuid,
    name:String,
    created_at:DateTime<Utc>,
}



