use axum::{
    Json,
    http::Response,
    extact::Json::ExtractJson,
    Router,
    response::IntoResponse,
    routing::{get,post},
};
use serde::{Serialize,Deserialize};
use Uuid::uuid;
use chrono::Utc;
use tokio::fs;
use std::error::Error;
use std::net::SocketAddr;



#[derive(Debug,Serialize)]
struct LoginReponse{
    status:String,
    message:String,
    reponded_at:Utc::now(),
}

#[derive(Debug,Deserialize)]
struct LoginRequest{
    id:Uuid,
    username:String,
    password:String,
    created_at:Utc::now(),
}

async fn log_in(ExtractJson(payload):ExtractJson<LoginRequest>)-> Impl IntoResponse{
    if payload.username == "David" && payload.password == "David123" {
        let response = LoginReponse{
            status:"Success".to_string(),
            message:"Successfully Signed in".to_string(),
        };
        Json(response)
    } else {
        let response = LoginReponse{
            status:"Failed".to_string(),
            message:"Username or password is Incorrect".to_string(),
        };
        Json(response)
    }
}



#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
let app = Router::new().route("/login",get(log_in));

let addr = SocketAddr::from(([127,0,0,1],3000));

let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

axum::serve(listener,app).await.unwrap();
  Ok(())
}

