use axum::{
    Json,
    Router,
    routing::{get,post}
};
use serde::{Serialize,Deserialize};
use std::net::SocketAddr;
use serde_json;
use tokio;
use uuid::Uuid;
use chrono::{DateTime,Utc};


#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/receive",post(send_msg));

}

#[derive(Serialize,Deserialize)]
struct Msg{
    id:Uuid,
    message:String,
    sent_at:DateTime<Utc>

}

#[derive(Serialize,Deserialize,Debug)]
struct Msg_Response{
    message:String,
    received_at:DateTime<Utc>
}


async fn send_msg(Json(payload):Json<Msg>)->()