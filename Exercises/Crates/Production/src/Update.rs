async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *"
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}
