use async_graphql::{Context, Object};
use sqlx::PgPool;
use crate::{models::{NewUser, User}};


pub struct MutationRoot;


#[Object]
impl MutationRoot {
async fn create_user(&self, ctx: &Context<'_>, name: String, email: String) -> async_graphql::Result<User> {
let pool = ctx.data::<PgPool>()?;
let new = NewUser { name, email };
let user = crate::db::create_user(pool, new).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;


// publish to subscriptions if needed (we'll use a broadcaster in schema.rs)
if let Some(broadcaster) = ctx.data_opt::<tokio::sync::broadcast::Sender<User>>() {
// ignore send error when no receivers
let _ = broadcaster.send(user.clone());
}


Ok(user)
}
}