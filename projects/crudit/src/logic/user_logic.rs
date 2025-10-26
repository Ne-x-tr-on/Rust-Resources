use crate::models::user::User;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn create_user(pool:&Pool<Postgres>,user:&User){
  sqlx::query(
    "INSERT INTO users(id,name,age) VALUES ($1,$2,$3)"
  )
  .bind(user.id)
  .bind(&user.name)
  .bind(user.age as i32)
  .execute(pool)
  .await
  .expect("Failed to inser user");
  println!("User {} inserted successfully",user.name)
}