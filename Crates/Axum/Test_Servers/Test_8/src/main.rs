use axum::{
    response::IntoResponse, routing::{get,post}, Json, Router
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Student{
    name:String,
    age:i32,
}

async fn student_handler()-> impl IntoResponse {
    let student = Student{
        name:"Newton".to_string(),
        age:19,
    };
    Json(student)
}



#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/", get(student_handler));

let addr = SocketAddr::from(([127,0,0,1],3000));
println!("Server Running on port:\n{:?}",addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
axum::serve(listener,app).await.unwrap();
}

