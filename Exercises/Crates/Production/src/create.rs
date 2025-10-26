async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}
