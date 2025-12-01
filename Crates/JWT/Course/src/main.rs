// fn main() {
//     // println!("https://www.youtube.com/watch?v=n2M4A4mO0QU");
//     // println!("    https://www.youtube.com/watch?v=_ccDqRTx-JU");
// }


use std::net::SocketAddr;

use axum::routing::post;
use serde::{Serialize,Deserialize};
use tokio;
use axum::{
    routing::get,Router,Json,
};
use std::error::Error;

pub mod model;
pub mod controller;
use crate::controller::login_handler;
use crate::controller::get_info_handler;



#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
   let app = Router::new()
   .route("/login",post(login_handler))
   .route("/info",get(get_info_handler));

   let addr = SocketAddr::from(([127,0,0,1],3000));
   println!("Server Running on Port {}",addr);

   let listener =tokio::net::TcpListener::bind(&addr).await.unwrap();

   axum::serve(listener,app).await.unwrap();
   Ok(())
}

