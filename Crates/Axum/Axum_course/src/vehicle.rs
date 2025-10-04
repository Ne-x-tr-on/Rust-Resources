use serde::{Serialize,Deserialize};
use axum::{response::IntoResponse,Json};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
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

// pub async fn vehicle_post() -> impl IntoResponse {
//   let response = serde_json::json!({
//     "status":"success",
//     "message":"Say Cheese Dodge Demon"
//   });
//   Json(response)
// }

pub async fn vehicle_post(Json(mut v):Json<Vehicle>)->Json<Vehicle>{
    println!(
        "Manufacture: {}, model: {}, year: {}",
        v.manufacturer, v.model, v.year
    );
    v.id = Some(uuid::Uuid::new_v4().to_string());

    Json::from(v)
}