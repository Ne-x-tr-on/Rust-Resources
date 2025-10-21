use serde::{Serialize,Deserialize};
use chrono::Utc;
use::uuid::Uuid;

#[derive(debug,FromRow)]
struct User{
  id:uuid,
  firstname:String,
  secondname:String,  
}