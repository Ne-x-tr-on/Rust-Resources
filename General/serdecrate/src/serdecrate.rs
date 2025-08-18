use serde::{Deserialize,Serialize};
use serde_json::to_string;

#[derive(Serialize,Deserialize)]
struct Dog{
  name:String,
  year_born:i32,
}

// use serde::{Serialize, Deserialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)] // <-- added Deserialize here
// struct Person {
//     name: String,
//     age: i32,
//     gender: String,
// }

// fn main() {
//     let person = Person {
//         name: "Newton Kamau".to_string(),
//         age: 19,
//         gender: "male".to_string(),
//     };

//     // Serialize to JSON string
//     let serialized = serde_json::to_string(&person).unwrap();
//     println!("{}", serialized);

//     // Deserialize JSON string back to struct
//     let deserialized: Person = serde_json::from_str(&serialized).unwrap();
//     println!("{:?}", deserialized);
// }

