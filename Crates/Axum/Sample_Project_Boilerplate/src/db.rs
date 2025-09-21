use sqlx::PgPool;
use dotenvy::var;
use anyhow::Result;

pub async fn init_pool() -> Result<PgPool> {
    let database_url = var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

// Example repository functions
use crate::models::{NewUser, User, NewStudent, Student};
use uuid::Uuid;
use chrono::Utc;
use sqlx::Row;

pub async fn create_user(pool: &PgPool, new: &NewUser, password_hash: &str) -> Result<User, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, name, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, password_hash, name, created_at
        "#,
        Uuid::new_v4(),
        new.email,
        password_hash,
        new.name,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: rec.id,
        email: rec.email,
        password_hash: rec.password_hash,
        name: rec.name,
        created_at: rec.created_at,
    })
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"SELECT id, email, password_hash, name, created_at FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| User {
        id: r.id,
        email: r.email,
        password_hash: r.password_hash,
        name: r.name,
        created_at: r.created_at,
    }))
}

pub async fn create_student(pool: &PgPool, new: &NewStudent) -> Result<Student, sqlx::Error> {
    let id = Uuid::new_v4();
    let created_at = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO students (id, name, class, admission_number, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        id,
        new.name,
        new.class,
        new.admission_number,
        created_at
    )
    .execute(pool)
    .await?;

    Ok(Student {
        id,
        name: new.name.clone(),
        class: new.class.clone(),
        admission_number: new.admission_number.clone(),
        created_at,
    })
}

pub async fn get_student(pool: &PgPool, id: Uuid) -> Result<Option<Student>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"SELECT id, name, class, admission_number, created_at FROM students WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| Student {
        id: r.id,
        name: r.name,
        class: r.class,
        admission_number: r.admission_number,
        created_at: r.created_at,
    }))
}
