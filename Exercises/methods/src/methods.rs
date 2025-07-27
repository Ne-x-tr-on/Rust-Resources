// axum_prerequisites.rs
// ================================================
// ✅ AXUM PREREQUISITES - Learn These Before Axum
// ================================================

fn main() {
    println!("🦀 Welcome to the Axum Prerequisites Practice File!");
    
    // ===== 1. DATA TYPES & METHODS =====
    // Strings
    let mut s = String::from("Hello");
    s.push('!');
    s.push_str(" Rustacean");
    println!("{}", s.replace("Rustacean", "Axum Dev"));

    // Vectors
    let mut v = vec![1, 2, 3];
    v.push(4);
    for num in &v {
        println!("Vec item: {}", num);
    }

    // Option and Result Methods
    let some_val: Option<i32> = Some(10);
    println!("Is some? {}", some_val.is_some());
    println!("Unwrapped: {}", some_val.unwrap_or(0));

    let result: Result<i32, &str> = Ok(5);
    println!("Result is ok? {}", result.is_ok());

    // ===== 2. CONTROL FLOW =====
    let score = 65;
    if score >= 80 {
        println!("Excellent!");
    } else if score >= 50 {
        println!("Pass");
    } else {
        println!("Fail");
    }

    // ===== 3. FUNCTIONS =====
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    println!("{}", greet("Newton"));

    // ===== 4. ENUMS + MATCH =====
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light = TrafficLight::Green;
    match light {
        TrafficLight::Red => println!("STOP"),
        TrafficLight::Yellow => println!("READY"),
        TrafficLight::Green => println!("GO"),
    }

    // ===== 5. STRUCTS =====
    struct User {
        name: String,
        age: u8,
    }

    let user1 = User {
        name: String::from("Newton"),
        age: 18,
    };

    println!("{} is {} years old.", user1.name, user1.age);

    // ===== 6. CLOSURES =====
    let add = |x: i32, y: i32| x + y;
    println!("Closure result: {}", add(5, 3));

    // ===== 7. TRAITS =====
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 3, y: 4 };
    let p2 = p1; // Copy
    println!("Point p2: {:?}", p2);

    // ===== 8. ERROR HANDLING =====
    fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    match safe_divide(10, 2) {
        Ok(val) => println!("Division result: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // ===== 9. MODULES & CRATES =====
    // See how modules are structured in larger apps.
    // For now, everything is in main.rs

    // ===== 10. ASYNC & TOKIO =====
    // (This block won't compile unless under #[tokio::main])
    /*
    #[tokio::main]
    async fn run_async_example() {
        use tokio::time::{sleep, Duration};
        println!("Waiting...");
        sleep(Duration::from_secs(1)).await;
        println!("Done!");
    }
    */

    // ===== 11. SERDE (for Axum JSON) =====
    // #[derive(Serialize, Deserialize)]
    // struct Person {
    //     name: String,
    //     age: u8,
    // }

    // let json = serde_json::to_string(&person).unwrap();
    // let parsed: Person = serde_json::from_str(&json).unwrap();

    println!("✅ You're on track to start building with Axum!");
}

// To go further, create folders like:
// src/practice/datatypes.rs
// src/practice/functions.rs
// ... and add detailed exercises in each.

// 50+ Rust methods demonstration across Vec, String, HashMap, Option, Result, etc.

use std::collections::HashMap;

fn main() {
    // === Vec<T> methods ===
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec: {:?}", vec);
    vec.pop();
    println!("vec after pop: {:?}", vec);
    println!("len: {}", vec.len());
    println!("is_empty: {}", vec.is_empty());
    vec.insert(1, 10);
    println!("after insert: {:?}", vec);
    vec.remove(0);
    println!("after remove: {:?}", vec);
    vec.clear();
    println!("after clear: {:?}", vec);

    // === String methods ===
    let mut s = String::from("Rust");
    s.push('y');
    s.push_str(" is fast!");
    println!("String: {}", s);
    println!("len: {}", s.len());
    println!("contains 'fast': {}", s.contains("fast"));
    println!("starts_with 'Rusty': {}", s.starts_with("Rusty"));
    println!("ends_with '!': {}", s.ends_with("!"));
    let replaced = s.replace("fast", "awesome");
    println!("replaced: {}", replaced);
    println!("to_uppercase: {}", s.to_uppercase());

    // === HashMap methods ===
    let mut map = HashMap::new();
    map.insert("lang", "Rust");
    map.insert("speed", "fast");
    println!("map: {:?}", map);
    println!("get lang: {:?}", map.get("lang"));
    println!("contains_key 'speed': {}", map.contains_key("speed"));
    map.remove("speed");
    println!("after remove: {:?}", map);

    // === Option methods ===
    let some_val = Some(10);
    println!("is_some: {}", some_val.is_some());
    println!("unwrap: {}", some_val.unwrap());
    println!("map +1: {:?}", some_val.map(|x| x + 1));
    println!("unwrap_or 0: {}", some_val.unwrap_or(0));

    let none_val: Option<i32> = None;
    println!("is_none: {}", none_val.is_none());
    println!("unwrap_or 5: {}", none_val.unwrap_or(5));

    // === Result methods ===
    let ok: Result<i32, &str> = Ok(100);
    println!("is_ok: {}", ok.is_ok());
    println!("unwrap: {}", ok.unwrap());
    println!("map x2: {:?}", ok.map(|x| x * 2));
    println!("unwrap_or 0: {}", ok.unwrap_or(0));

    let err: Result<i32, &str> = Err("Oops");
    println!("is_err: {}", err.is_err());
    println!("unwrap_or 0: {}", err.unwrap_or(0));

    // === &str methods ===
    let str_slice = "Hello Rust";
    println!("len: {}", str_slice.len());
    println!("contains 'Rust': {}", str_slice.contains("Rust"));
    println!("split: {:?}", str_slice.split(' ').collect::<Vec<_>>());
    println!("trim: '{}'", "  padded ".trim());
    println!("replace: {}", str_slice.replace("Rust", "World"));
    println!("starts_with 'Hello': {}", str_slice.starts_with("Hello"));
    println!("ends_with 'Rust': {}", str_slice.ends_with("Rust"));
    println!("to_lowercase: {}", str_slice.to_lowercase());
}
