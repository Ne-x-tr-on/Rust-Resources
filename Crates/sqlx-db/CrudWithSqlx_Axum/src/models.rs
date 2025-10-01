use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
