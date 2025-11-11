use serde::{Deserialize,Serialize};
use uuid::Uuid;
use chrono::{DateTime,Utc};
use sqlx::FromRow;

#[derive(Deserialize,Serialize,FromRow)]
pub struct Developer{
  pub id:Uuid,
  pub name:String,
  pub created_at:DateTime<Utc>,
}