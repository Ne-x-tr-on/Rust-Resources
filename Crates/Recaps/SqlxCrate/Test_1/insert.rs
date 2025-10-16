use sqlx::{PgPool,Error};
use crate::model::User;

pub async fn insert_user(pool:&PgPool,username:&str,email:&str,gender:&str)->Result<User,sqlx::Error>{
  let user = sqlx::query_as::<_,user>(
    "INSERT INTO users (username,email,gender)
    VALUES($1,$2,$3)
    RETURNING id,username,email,gender"
  )
  .bind(username)
  .bind(email)
  .bind(gender)
  .fetch_one(&pool)
  .await?;

  Ok(user)
}