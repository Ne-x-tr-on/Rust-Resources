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
// #[tokio::main]
// async fn main(){
//     let app = Router::new().
//     route("/",get(||async {"Hello World"}));

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

//     axum::serve(listener,app)
//     .await
//     .unwrap();
// }