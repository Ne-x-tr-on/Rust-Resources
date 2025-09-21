// use askama::Template;
// use axum::{
//     response::Html,
//     routing::get,
//     Router,
// };

// #[derive(Template)]
// #[template(path = "signup.html")]
// struct SignupTemplate;

// async fn signup_page() -> Html<String> {
//     let template = SignupTemplate;
//     Html(template.render().unwrap())
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(signup_page));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }


use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate;

// This struct will hold signup form data
#[derive(Deserialize)]
struct SignupForm {
    username: String,
    password: String,
}

// Renders the signup page
async fn signup_page() -> Html<String> {
    let template = SignupTemplate;
    Html(template.render().unwrap())
}

// Handles form submission, responds with JSON
async fn signup_submit(Form(data): Form<SignupForm>) -> Json<serde_json::Value> {
    if data.username.is_empty() || data.password.is_empty() {
        Json(json!({
            "status": "error",
            "message": "Username and password cannot be empty"
        }))
    } else {
        // Normally, save to DB here
        Json(json!({
            "status": "success",
            "message": format!("User '{}' registered successfully!", data.username)
        }))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/signup", get(signup_page).post(signup_submit));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
