// use axum::{routing::get, Router};
// use std::net::SocketAddr;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() {
//     // define routes
//     let app = Router::new().route("/", get(|| async { "Hello, Neza CEO and Founder" }));

//     // bind a TCP listener
//     let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

//     println!("🚀 Server running on http://{}", listener.local_addr().unwrap());

//     // serve using axum::serve
//     axum::serve(listener, app)
//         .await
//         .unwrap();
// }


// use axum::{
//     Router,
//     routing::get,
// };
// use tokio::net::TcpListener;
// #[tokio::main]
// async fn main(){
//     let app = Router::new().
//     route("/",get(||async {"Welcome to Axum"}));

//     let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     println!("Server Running on port:\n{}",listener.local_addr().unwrap());

//     axum::serve(listener,app)
//     .await
//     .unwrap();
//     // println!("Server Running on port:\n{}",listener.local_addr().unwrap());
// }

use axum::{
    routing::{get,post},
    Router,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    // let app = Router::new()
    let app = Router::new()
    .route("/vehicle", get(vehicle_get).post(vehicle_post))
    .route("/bank_form", post(bank_form).post(login));
//    post(login));

  
    // .route("/vehicle",get(vehicle_get))
    // .route("/vehicle",post(vehicle_get));
    
 let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
 println!("Server running on port:\n{:?}",addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(listener,app).await.unwrap();

}
#[derive(Serialize)]
struct Vehicle{
    manufacturer:String,
    model:String,
    year:u32,
    id:String,
}


async fn vehicle_get()-> impl IntoResponse{
    // println!("Welcome to the Vehicle Route")
  let vehicle = Vehicle{
    manufacturer:"Dodge".to_string(),
    model:"Corolla".to_string(),
    year:2020,
    id:uuid::Uuid::new_v4().to_string(),
  };
  Json(vehicle)
}

async fn vehicle_post()-> impl IntoResponse{
    // println!("Request Received from Vehicle Route")
}


#[derive(Serialize)]
struct Form{
    name:String,
    gender:String,
    phone_no:i32,
}
async fn bank_form() -> impl IntoResponse{
    let test_form = Form{
        name:"Benjamin".to_string(),
        gender:"Male".to_string(),
        phone_no:0701090807,
    };
    Json(test_form)
}

#[derive(Serialize)]
struct Signup{
    name:String,
    password:String,
}

async fn login() -> impl IntoResponse{
    let loginform = Signup{
        name:"Newton Kamau".to_string(),
        password:"password123".to_string(),
    };
    Json(loginform)
}