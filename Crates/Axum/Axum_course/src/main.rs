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
    routing::get,
    Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/",get(|| async {
        "Sample Simple Axum Server"
    }));
    
 let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
 println!("Server running on port:\n{:?}",addr);

let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(listener,app).await.unwrap();
}

