// === Rust Async, Error Handling, Closures, and Project Management Course ===

// -----------------------------
// MODULE 1: Async and Await
// -----------------------------
// To run this example, add tokio to your dependencies in Cargo.toml:
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

async fn do_work() -> u32 {
    sleep(Duration::from_secs(2)).await;
    println!("Finished work!");
    42
}

#[tokio::main]
async fn main_async() {
    let result = do_work().await;
    println!("Result: {}", result);
}

// -----------------------------
// MODULE 2: Error Handling
// -----------------------------
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main_error() {
    match read_file() {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

// -----------------------------
// MODULE 3: Closures
// -----------------------------
fn main_closure() {
    let add = |a: i32, b: i32| a + b;
    println!("5 + 3 = {}", add(5, 3));

    let mut counter = 0;
    let mut inc = || {
        counter += 1;
        counter
    };
    println!("Counter: {}", inc());
    println!("Counter: {}", inc());
}

// -----------------------------
// MODULE 4: Managing Projects
// -----------------------------
// File: src/main.rs
mod utils;

fn main_project() {
    utils::greet("Kamau");
}

// File: src/utils.rs
// pub fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }

// To run: 
// 1. cargo new project_name
// 2. Place utils.rs inside src/
// 3. Import with mod utils; and call with utils::greet()
// 4. Run with: cargo run

// -----------------------------
// MODULE 5: Using Crates
// -----------------------------
// Add to Cargo.toml:
// reqwest = { version = "0.11", features = ["blocking"] }

use reqwest::blocking::get;

fn fetch_web() -> Result<String, Box<dyn std::error::Error>> {
    let resp = get("https://httpbin.org/ip")?.text()?;
    Ok(resp)
}

fn main_crates() {
    match fetch_web() {
        Ok(data) => println!("Fetched: {}", data),
        Err(e) => eprintln!("Fetch error: {}", e),
    }
}

// -----------------------------
// RUNNING THE COURSE:
// Uncomment each main function to test separately
// - main_async()
// - main_error()
// - main_closure()
// - main_project()
// - main_crates()
