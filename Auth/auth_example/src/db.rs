use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    Ok(pool)
}
