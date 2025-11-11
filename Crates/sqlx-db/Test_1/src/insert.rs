use sqlx::PgPool;
use crate::model::Profile;

pub async fn intert_profile(pool:&PgPool,user_name:&str,full_name:&str,email:&str)->Result<Profile,sqlx::Error>{
  sqlx::query_as::<_,Profile>(
    r#"
    INSERT INTO entityid (user_name,full_name,email)
    VALUES ($1,$2,$3)
    RETURNING (user_name,full_name,email)
    "#
  )
  .bind(user_name)
  .bind(full_name)
  .bind(email)
  .fetch_one(pool)
  .await
}
