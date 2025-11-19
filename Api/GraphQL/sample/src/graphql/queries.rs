use async_graphql::{Context, Object};
use sqlx::PgPool;
use crate::models::User;


pub struct QueryRoot;


#[Object]
impl QueryRoot {
async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
let pool = ctx.data::<PgPool>()?;
let users = crate::db::get_users(pool).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
Ok(users)
}
}