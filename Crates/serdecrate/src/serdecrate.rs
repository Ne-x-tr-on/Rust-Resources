use serde::{Serialize,Deserialize};
use serde_json::to_string;

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Dog{
  pub name:String,
  pub year_born:i32,
  pub owner:DogOwner,
}


#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DogOwner{
  pub first_name:String,
  pub last_name:String,
}