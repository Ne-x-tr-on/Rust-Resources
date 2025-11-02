use serde::{de, Deserialize, Serialize};
use tokio;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug,Deserialize,Serialize)]
struct User{
    id:Uuid,
    username:String,
    userage:i32,
}


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{

let new_user = User{
    id:Uuid::new_v4(),
    username:"Newton".to_string(),
    userage:19,
};
println!("Original User:\n{:?}",new_user);

// Serialization
let json_string = serde_json::to_string_pretty(&new_user)?;
println!("Serialized User:\n{:?}",json_string);

// Deserialization
let deserialization:User = serde_json::from_str(&json_string)?;
println!("Deserialized User:\n{:?}",deserialization);

    Ok(())
}
