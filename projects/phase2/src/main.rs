use axum::{
    routing::get,
    response::Json,
    Router,
};
use uuid::Uuid;
use serde::Serialize;
use std::net::SocketAddr;


#[derive(Serialize)]
struct User{
    id:Uuid,
    name:String,
    email:String,
}

async fn get_user()->Json<User>{
    let user = User{
        id:Uuid::new_v4(),
        name:"Newton Manyeki".to_string(),
        email:"newton@gmail.com".to_string(),
    };
    Json(user)
}



#[tokio::main]
async fn main(){
    let app = Router::new()
     .route("/login/password",get(get_user));

    let addr = SocketAddr::from(([127,0,0,2],3000));
    println!("Server Running on https://{:?}",addr);

    let listerner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listerner,app).await.unwrap();
}
