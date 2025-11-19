use async_graphql::{Schema, EmptySubscription};
use crate::graphql::{queries::QueryRoot, mutations::MutationRoot, subscriptions::SubscriptionRoot};
use sqlx::PgPool;
use tokio::sync::broadcast;
use crate::models::User;


pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;


pub fn build_schema(pool: PgPool) -> AppSchema {
// broadcaster for subscriptions (small example)
let (tx, _rx) = broadcast::channel::<User>(100);


Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
.data(pool)
.data(tx)
.finish()
}