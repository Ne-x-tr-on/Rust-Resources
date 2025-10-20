use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::MigrateDatabase;
use sqlx::Row; // For getting values from rows
use uuid::Uuid; // For IDs
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("Failed to get the Db Url");

    // 1️⃣ Create the pool (reference-counted, clonable)
    let pool = PgPoolOptions::new()
        .max_connections(10) // good for concurrent scalable systems
        .connect("databse_url")
        .await?;

          // Auto-create DB if missing
    if !sqlx::Postgres::database_exists(database_url).await? {
        sqlx::Postgres::create_database(database_url).await?;
        println!("Database created automatically");
    }
    
    // ✅ Autorun migrations
    sqlx::migrate!("./migrations") // your folder with migration SQL files
        .run(&pool)
        .await?;
    println!("Migrations applied automatically on startup");


    // ===== CREATE =====
    let insert_pool = pool.clone(); // clone pool handle
    let book_id = Uuid::new_v4();
    sqlx::query("INSERT INTO books (id, title, author, published_at) VALUES ($1, $2, $3, $4)")
        .bind(book_id)
        .bind("Rust for Beginners")
        .bind("Newton")
        .bind(Utc::now())
        .execute(&insert_pool)
        .await?;
    println!("Inserted book with id: {}", book_id);

    // ===== READ =====
    let query_pool = pool.clone();
    let row = sqlx::query("SELECT id, title, author, published_at FROM books WHERE id = $1")
        .bind(book_id)
        .fetch_one(&query_pool)
        .await?;
    let title: &str = row.get("title");
    let author: &str = row.get("author");
    println!("Read book: {} by {}", title, author);

    // ===== UPDATE =====
    let update_pool = pool.clone();
    sqlx::query("UPDATE books SET title = $1 WHERE id = $2")
        .bind("Advanced Rust")
        .bind(book_id)
        .execute(&update_pool)
        .await?;
    println!("Updated book title for id: {}", book_id);

    // ===== DELETE =====
    let delete_pool = pool.clone();
    sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(book_id)
        .execute(&delete_pool)
        .await?;
    println!("Deleted book with id: {}", book_id);

    Ok(())
}




// CREATE TABLE example (
//     id UUID PRIMARY KEY,
//     name VARCHAR(50),
//     description TEXT,
//     age INT,
//     balance DECIMAL(10,2),
//     is_active BOOLEAN,
//     created_at TIMESTAMPTZ,
//     metadata JSONB
// );

use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Example {
    pub id: Uuid,                  // UUID -> uuid::Uuid
    pub name: String,              // VARCHAR -> String
    pub description: String,       // TEXT -> String
    pub age: i32,                  // INT -> i32
    pub balance: f64,              // DECIMAL -> f64 (or use rust_decimal crate for precision)
    pub is_active: bool,           // BOOLEAN -> bool
    pub created_at: DateTime<Utc>, // TIMESTAMPTZ -> chrono::DateTime<Utc>
    pub metadata: Value,           // JSONB -> serde_json::Value
}


use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use serde_json::Value;

let my_int: i32 = 42;                      // INT
let my_smallint: i16 = 300;                // SMALLINT
let my_bigint: i64 = 9000000000;           // BIGINT
let my_decimal: f64 = 123.45;              // DECIMAL
let my_float: f32 = 3.14;                  // REAL
let my_double: f64 = 3.141592;             // DOUBLE PRECISION

let my_char: String = "A".to_string();     // CHAR(1)
let my_varchar: String = "Hello".to_string(); // VARCHAR
let my_text: String = "Long text here".to_string(); // TEXT

let my_date: NaiveDate = NaiveDate::from_ymd(2025,10,20); // DATE
let my_time: NaiveTime = NaiveTime::from_hms(14,30,0);    // TIME
let my_timestamp: NaiveDateTime = NaiveDateTime::from_timestamp(1_693_971_000,0); // TIMESTAMP
let my_timestamptz: DateTime<Utc> = Utc::now();           // TIMESTAMPTZ

let my_bool: bool = true;                 // BOOLEAN
let my_uuid: Uuid = Uuid::new_v4();       // UUID
let my_json: Value = serde_json::json!({"name": "Newton"}); // JSON / JSONB

