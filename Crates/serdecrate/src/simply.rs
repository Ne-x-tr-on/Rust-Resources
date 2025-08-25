use serde::{Serialize, Deserialize}; // bring in Serde traits
use serde_json; // for JSON handling

// Define a struct that can be serialized & deserialized
#[derive(Serialize, Deserialize, Debug)]
struct Dog {
    name: String,
    age: u8,
}

fn main() {
    // JSON string (coming from file, API, etc.)
    let json_str = r#"
        {
            "name": "Tommy",
            "age": 3
        }
    "#;

    // 1️⃣ Deserialize JSON → Rust struct
    let dog: Dog = serde_json::from_str(json_str).unwrap();
    println!("Deserialized: {:?}", dog);

    // 2️⃣ Serialize Rust struct → JSON
    let dog_json = serde_json::to_string(&dog).unwrap();
    println!("Serialized: {}", dog_json);
}
