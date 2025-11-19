use async_graphql::{Subscription, Context};
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use crate::models::User;


pub struct SubscriptionRoot;


#[Subscription]
impl SubscriptionRoot {
async fn user_created(&self, ctx: &Context<'_>) -> impl Stream<Item = User> {
let rx = ctx.data::<tokio::sync::broadcast::Sender<User>>().unwrap().subscribe();
tokio_stream::wrappers::BroadcastStream::new(rx)
.filter_map(|res| async move { res.ok() })
}
}