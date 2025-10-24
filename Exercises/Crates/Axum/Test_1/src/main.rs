use axum::{
    routing::{get,post},
    Router,
    Json,
    http::StatusCode,
    response::IntoResponse,
};

use std::net::SocketAddr;
use serde_json::json;

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/", get(get_user));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server Running on http://{}",addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();

}

async fn get_user() -> impl IntoResponse{
    let user = json!({
        "id":1,
        "name":"Newton",
        "role":"Mechatronics Student"
    });
   (StatusCode::OK,Json(user))
}