// use std::fmt::format;

// use axum::{Router,routing::get,extract::Path,Json};
// use serde::Serialize;

// #[derive(Serialize)]
// struct UserResponse{
//     id:u32,
//     message:String,
// }

// async fn get_user(Path(User_id):Path<u32>)-> Json<UserResponse>{
//     let resp = UserResponse{
//         id:User_id,
//         message:format!("User {} Found",User_id),
//     };
//     Json(resp)
// }

// async fn get_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
//     format!("User {} → Post {}", user_id, post_id)
// }

// fn app() -> Router {
//     Router::new().route("/users/:user_id/posts/:post_id", get(get_post))
// }

// use std::sync::Arc;

// use axum::routing::get;
// use axum::Router;

// #[derive(Clone)]
// struct AppState{
//     app_name:String,
//     counter:u32,
// }

// fn app()->Router{
//     let state = AppState{
//         app_name:"Newton API".to_string(),
//         counter:1,
//     };

//     Router::new()
//     .route("/",get(handler))
//     .with_state(Arc::new(state));
// }
use serde::Serialize;
use tokio;
use tracing_subscriber;

fn init_logging() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();
    let newUser = User {
        name: "David".to_string(),
    };
    tracing::info!("Starting Newton's API...");
    tracing::info!("New User Created {}", newUser.name);
}

#[derive(Serialize)]
struct User {
    name: String,
}
