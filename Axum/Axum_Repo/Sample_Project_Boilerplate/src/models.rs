use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub class: String,
    pub admission_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub class: String,
    pub admission_number: String,
    pub created_at: DateTime<Utc>,
}
