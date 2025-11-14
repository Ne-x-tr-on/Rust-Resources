use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::{env, error::Error};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Developer {
    pub id: Uuid,
    pub name: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn insert_developer(
    pool:&PgPool,
    id:Uuid,
    name:&str,
    password:Option<&str>,
    created_at:DateTime<Utc>
)->Result<Developer,sqlx::Error>{
    let sql = r#"
    INSERT INTO developers (id,name,password,created_at) VALUES($1,$2,$3,$4)
    RETURNING (id,name,password,creates_at)
    "#;

    let developer = sqlx::query_as::<_,Developer>(sql)
    .bind(id)
    .bind(name)
    .bind(password)
    .bind(created_at)
    .fetch_one(pool)
    .await?;
    Ok(developer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB path not found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let new_dev = insert_developer(
        &pool,
        Uuid::new_v4(),
        "Newton Kamau",
        Some("supersecurepassword"),
        Utc::now(),
    )
    .await?;

    println!("✅ Inserted developer: {:?}", new_dev);

    Ok(())
}
