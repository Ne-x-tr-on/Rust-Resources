// use axum::{
//     response::IntoResponse,
//     routing::{get,post},
//     Router,
//     Json
// };

// use serde::{Deserialize,Serialize};
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main(){
//     let app = Router::new()
//                 .route("/",post(post_struct));

//     let addr = SocketAddr::from(([127,0,0,1],9000));
//     println!("Server running on port: \n{:?}",addr);

//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     axum::serve(listener,app).await.unwrap();
// }

// #[derive(Deserialize,Serialize,Debug)]
// struct Data{
//     name:String,
//     age:i32,
// }

// async fn post_struct()-> impl IntoResponse{
//     let formdata = Data{
//         name:"Newton".to_string(),
//         age:19
//     };
//     Json(formdata)
// }



use axum::{
    response::IntoResponse, routing::post, Json, Router
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(post_struct));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    name: String,
    age: i32,
}

async fn post_struct(Json(payload):Json<Data>) ->  impl IntoResponse{
    println!("Received Data:\n{:?}",payload);
    let response = Data{
        name:payload.name,
        age:payload.age
    };
    Json(response)
}
