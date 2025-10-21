let pool = PgPoolOptions::new()
  .max_connections(10)
  .connect(&url)
  .await;

  pub async fn create_user(
    pool:&PgPool,
    name:&str,
    username:&str,
  )-> Result<(),sqlx::Error>{
    let user_id = Uuid::new_v4();
    let insert_pool = pool.clone();

    sqlx::query(
      "INSERT INTO User (id,name,username,created_at),VALUES($1,$2,$3)"
    )
    .bind(user_id)
    .bind(name)
    .bind(username)
    .bind(Utc::now)
    .execute(insert_pool)
    .await?;
    Ok(())
  }