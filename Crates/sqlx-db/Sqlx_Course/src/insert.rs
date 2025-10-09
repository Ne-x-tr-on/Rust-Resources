use sqlx::PgPool;
use crate::model::EntityId;

pub async fn insert_profile(pool:&PgPool,username:String,full_name:String,email:String) -> Result<EntityId,sqlx::Error>{
  sqlx::query_as::<sqlx::postgres::Postgres,EntityId>(
    r"
    insert into profile
    (user_name,full_name,email)
    values
    ($1,$2,$3)
    returning id
    ")
  .bind(username)
  .bind(full_name)
  .bind(email)
  .fetch_one(pool)
  .await
}