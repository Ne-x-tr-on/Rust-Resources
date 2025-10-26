use std::error::Error;

mod models;
mod db;
mod logic;
use crate::db::create_pool;
use crate::models::user::User;
use crate::logic::user_logic::create_user;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
 let pool = create_pool().await;
 
 let new_user = User::new(1,"Newton",19);
 create_user(&pool,&new_user).await;
  Ok(())
}


