async fn delete_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> StatusCode {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;
    
    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
