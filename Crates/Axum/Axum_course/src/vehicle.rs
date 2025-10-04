// use serde::Serialize;
// use axum::{response::IntoResponse,Json};
// use uuid::Uuid;

// #[derive(Serialize)]
// pub struct Vehicle {
//     pub manufacturer: String,
//     pub model: String,
//     pub year: u32,
//     pub id: String,
// }

// pub async fn vehicle_get() -> impl IntoResponse {
//     let vehicle = Vehicle {
//        manufacturer: "Dodge".to_string(),
//        model: "Corolla".to_string(),
//        year: 2020,
//        id: Uuid::new_v4().to_string(),
//     };
//     Json(vehicle)
// }

// pub async fn vehicle_post() -> impl IntoResponse {
//     let response = serde_json::json!({
//         "status": "success",
//         "message": "Vehicle POST request received"
//     });
//     Json(response)
// }

use serde::Serialize;
use axum::{response::IntoResponse,Json};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Vehicle {
    pub manufacturer: String,
    pub model: String,
    pub year: u32,
    pub id: Option<String>,
}

pub async fn vehicle_get() -> Json<Vehicle> {
 Json::from(
     Vehicle {
       manufacturer: "Dodge".to_string(),
       model: "Corolla".to_string(),
       year: 2020,
       id: Some(Uuid::new_v4().to_string()),
    })
    // Json(vehicle)
}

pub async fn vehicle_post() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "success",
        "message": "Vehicle POST request received"
    });
    Json(response)
}