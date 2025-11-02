use serde::{Serialize, Deserialize}; 
#[allow(unused)]
use serde_json; 

#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    name: String,
    age: u8,
}

// fn main() {
//     // JSON string (coming from file, API, etc.)
//     let json_str = r#"
//         {
//             "name": "Tommy",
//             "age": 3
//         }
//     "#;

//     // 1️⃣ Deserialize JSON → Rust struct
//     let dog: Dog = serde_json::from_str(json_str).unwrap();
//     println!("Deserialized: {:?}", dog);

//     // 2️⃣ Serialize Rust struct → JSON
//     let dog_json = serde_json::to_string(&dog).unwrap();
//     println!("Serialized: {}", dog_json);
// }
