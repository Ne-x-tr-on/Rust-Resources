mod routes;
mod handlers;

use crate::routes::user;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = user::get_routes();

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on:\n{addr}")
    axum::serve(listener, app).await.unwrap();
}
