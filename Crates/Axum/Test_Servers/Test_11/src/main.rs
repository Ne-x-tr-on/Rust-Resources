use axum::{
    http::StatusCode, routing::{get, post}, Json, Router,
    response::IntoResponse,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, net::SocketAddr};
use tokio;

#[tokio::main]
async fn main() {
let app: Router<()> = Router::new()
    .route("/", get(root))
    .route("/hello/produc/helloagain", post(greet));

    // let pool = PgPoolOptions::new().
    // max_connections(5)
    // .connect(url)
    // .expect("Failed to connect to the database");

    let addr = SocketAddr::from(([127,0,0,1],3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();
}

#[derive(Serialize, Deserialize,Debug)]
struct greetinput {
    name: String,
}

#[derive(Deserialize, Debug,Serialize)]
struct greetResponse {
    name: String,
    created_at: DateTime<Utc>,
}

async fn root() -> &'static str {
    "Hello Mr"
}

// async fn greet(Json(payload):Json<greetinput>)->(StatusCode,Json<greetResponse>){
//     let response = greetResponse{
//         name:"David Kamau".to_string(),
//         created_at: Utc::now(),
//     };
//     (StatusCode::OK,Json(response))
// }

// async fn greet(Json(payload):Json<greetinput>)->Result<(StatusCode,Json<greetResponse>),StatusCode>{
//     let response = greetResponse { name: "Newton Kamau".to_string(), created_at: Utc::now() };
//     Ok((StatusCode::OK,Json(response)))
// }

#[allow(unused)]
async fn greet(Json(_payload):Json<greetinput>)->impl IntoResponse{
    let response = greetResponse{
        name:"Newton".to_string(),
        created_at:Utc::now(),
    };
    Json(response)
}
