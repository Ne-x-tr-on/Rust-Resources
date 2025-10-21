use axum::{
  Router,Json,
  routing::{get,delete,update,post},
  response::IntoResponse,
  http::StatusCode,
  extract::{Path,State}
}

use serde::{Deserialize,Serialize};
use sqlx::{PgPool,Postgres::PgPoolOptions};
use chrono::{Utc,DateTime};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
struct User{
  id:Uuid,
  name:String,
  username:String,
  create_at:DatTime<Utc>,
}
