use serde::{
  Serialize,
  Deserialize
};

use axum::{
  response::IntoResponse,
  Json,
  extract::Json as ExtractJson
};

#[derive(Deserialize)] 
pub struct LoginRequest {
   pub username: String,
   pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: String,
    pub message: String,
}

pub async fn login(ExtractJson(payload):ExtractJson<LoginRequest>) -> impl IntoResponse{
  if payload.username == "Newton" && payload.password=="Newton123"{
    let response = LoginResponse{
      status:"Success".to_string(),
      message:"Login Success".to_string(),
    };
    Json(response)
} else {
  let response = LoginResponse{
    status:"Failed".to_string(),
    message:"payload Error".to_string(),
  };
  Json(response)
}
}