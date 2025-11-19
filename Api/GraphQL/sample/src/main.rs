use axum::{routing::get, Router, extract::Extension, response::IntoResponse};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tracing_subscriber;
use dotenvy::dotenv;
use std::net::SocketAddr;


mod models;
mod db;
mod graphql;
mod schema;


use schema::build_schema;
use crate::schema::AppSchema;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
tracing_subscriber::fmt::init();
dotenv().ok();


let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let bind_addr: SocketAddr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()).parse().unwrap();


let pool = db::init_pool(&database_url).await?;


let schema = build_schema(pool);


let app = Router::new()
.route("/graphql", axum::routing::post(graphql_handler))
.route("/", get(graphiql))
.layer(Extension(schema));


println!("Server running at http://{}", bind_addr);
axum::Server::bind(&bind_addr)
.serve(app.into_make_service())
.await?;


Ok(())
}


async fn graphql_handler(Extension(schema): Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
schema.execute(req.into_inner()).await.into()
}


async fn graphiql() -> impl IntoResponse {
let html = async_graphql::http::playground_source(async_graphql::http::GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"));
axum::response::Html(html)
}