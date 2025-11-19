use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
pub id: i32,
pub name: String,
pub email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
pub name: String,
pub email: String,
}