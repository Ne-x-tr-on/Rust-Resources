// use axum::{
//     response::IntoResponse,
//     routing::{get, post},
//      Router
// };
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main(){
//     let app = Router::new()
//         .route("/",get(get_request))
//         .route("/",post(post_request));

// let addr = SocketAddr::from(([127,0,0,1],4000));
// println!("Server Running on: \n{:?}",addr);

// let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

// axum::serve(listener,app).await.unwrap();

// }

// async fn get_request() -> impl IntoResponse{
//    "Hello Mr Newton"
// }

// async fn post_request() -> impl IntoResponse{
//     "Hello Newton"
// }

// use std::io;

// fn main(){
//  println!("Guess a number from 0 -10");

//  let mut input = String::new();

//  io::stdin()
//         .read_line(&mut input)
//         .expect("Something Went Wrong");

//  let number:i32 = match input.trim().parse(){
//     Ok(num) => num,
//     Err(_) => {
//         println!("Enter a valid number");
//         return;
//     }
    
//  };

//  for i in 1..=10{
//     println!("{}",number*i);
//  }

// }



use axum::{
    response::IntoResponse,
     routing::{get, post},
      Router,
      Json
};
use std::net::SocketAddr;
use serde::{Deserialize,Serialize};


#[tokio::main]
async fn main(){
    let thisroutes = Router::new()
        .route("/",get(this_router_post))
        // .route("/", post(post_handler))
        .route("/student",get(post_student));

    let addr = SocketAddr::from(([127,0,0,1],7000));
    println!("Axum Server running on port:\nhttps:\\{:?}",addr);


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener,thisroutes).await.unwrap();

}

#[derive(Deserialize,Serialize,Debug)]
struct Student{
    name:String,
    gender:String,
}

#[allow(unused)]
async fn post_student() -> impl IntoResponse{
    let mut data = Student{
        name:"Newton".to_string(),
        gender:"male".to_string(),
    };
    Json(data)
}

async fn this_router_post() -> impl IntoResponse{
    "Hello from post router"
}


async fn post_handler() -> impl IntoResponse{
    "Hello from post_handler"
}

