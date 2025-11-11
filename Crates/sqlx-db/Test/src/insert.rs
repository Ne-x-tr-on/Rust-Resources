use sqlx::{PgPool,FromRow};
use uuid::Uuid;
use chrono::{DateTime,Utc};

use crate::model::Developer;

pub async fn insert_dev(pool:&PgPool,id:Uuid,name:&str,created_at:DateTime<Utc>)->Result<Developer,sqlx::Error>{
  sqlx::query_as::<_,Developer>(
    
    r#"
    INSERT INTO developer(id,name,created_at)
     VALUES ($1,$2,$3)
    RETURNING (id,name,created_at)
    "#)

    .bind(id)
    .bind(name)
    .bind(created_at)
    .fetch_one(pool)
    .await
}
