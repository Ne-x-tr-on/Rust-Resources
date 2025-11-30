use axum::{
    http::StatusCode, routing::{get,post},Json, Router
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime,Utc};
use tokio;
use std::error::Error;
use std::net::SocketAddr;


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    let app = Router::new()
    .route("/",post(post_handler));
let addr = SocketAddr::from(([127,0,0,1],3000));
println!("Server is Running on Port {:?}",addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(listener, app).await.unwrap();

Ok(())
}

#[derive(Debug, Deserialize)]
struct NewUser{
    name:String,
    age:i32,
}

#[derive(Debug,Serialize)]
struct NewUserResp{
    id:Uuid,
    name:String,
    age:i32,
    created_at:DateTime<Utc>,
}


async fn post_handler(Json(payload):Json<NewUser>)->(StatusCode,Json<NewUserResp>){
    let response = NewUserResp{
        id:Uuid::new_v4(),
        name:payload.name.clone(),
        age:payload.age.clone(),
        created_at:Utc::now(),
    };
    println!("---------");
    println!("New User {:?} \nUsers age: {:?}",payload.name,payload.age);
    println!("---------");
    (StatusCode::CREATED,Json(response))
}