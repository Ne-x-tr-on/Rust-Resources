use sqlx::PgPool;
use crate::models::{User, NewUser};


pub async fn init_pool(database_url: &str) -> anyhow::Result<PgPool> {
let pool = PgPool::connect(database_url).await?;
Ok(pool)
}


pub async fn get_users(pool: &PgPool) -> anyhow::Result<Vec<User>> {
let rows = sqlx::query!(r#"SELECT id, name, email FROM users ORDER BY id"#)
.fetch_all(pool)
.await?;


let users = rows.into_iter().map(|r| User { id: r.id, name: r.name, email: r.email }).collect();
Ok(users)
}


pub async fn create_user(pool: &PgPool, new: NewUser) -> anyhow::Result<User> {
let row = sqlx::query!(r#"INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"#, new.name, new.email)
.fetch_one(pool)
.await?;
Ok(User { id: row.id, name: row.name, email: row.email })
}