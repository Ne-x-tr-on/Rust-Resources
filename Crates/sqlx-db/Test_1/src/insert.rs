use sqlx::PgPool;
use crate::model::EntityId;

pub async fn insert_profile(pool:&PgPool,user_name:&str,full_name:&str)->Result<EntityId,sqlx::Error>{
  sqlx::query_as::<_,EntityId>(
    r#"
    INSERT INTO entityid (user_name,full_name) 
    VALUES($1,$2)
    RETURNING (user_name,full_name)
    "#
  )
  .bind(user_name)
  .bind(full_name)
  .fetch_one(pool)
  .await
}