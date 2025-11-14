use chrono::{DateTime, Utc};
use dotenvy::dotenv;
// use serde::de::Error;
use sqlx::{
    FromRow, PgPool,
    postgres::{self, PgPoolOptions},
};
use uuid::Uuid;
// use std::{env,error::Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, FromRow)]
struct Pet {
    id: Uuid,
    name: String,
    collar_no: Option<String>,
    created_at: DateTime<Utc>,
}

pub async fn newPet(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    collar_no: &str,
    created_at: DateTime<Utc>,
) -> Result<Pet, sqlx::Error> {
    let Petquerry = r#"
    INSERT INTO pet (id,name,collar_no,created_at)
    VALUES($1,$2,$3,$4)
    RETURNING(id,name,collar_no,created_at)
    "#;

    let pet = sqlx::query_as::<_, Pet>(Petquerry)
        .bind(id)
        .bind(name)
        .bind(collar_no)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
    Ok(pet)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("Check your Env Variable");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let mypet = newPet(&pool, Uuid::new_v4(), "Tommy", Some("SD11"), Utc::now());

    sqlx::migrate!("./migrations")
    .run(pool)
    .await?;

    Ok(())
}
