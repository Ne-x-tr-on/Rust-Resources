use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub jti_hash: String,
    pub expires_at: chrono::NaiveDateTime,
    pub revoked: bool,
    pub created_at: chrono::NaiveDateTime,
}
