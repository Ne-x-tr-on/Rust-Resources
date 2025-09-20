// use dotenv::dotenv;
// use std::env;

// fn main() {
//     // Load .env file
//     dotenv().ok();

//     // Access env variables
//    let db_url = env::var("DATABASE_URL").expect("Failed to read");
//    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());
//    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

//     println!("DB URL: {}", db_url);
//     println!("API Key: {}", api_key);
//     println!("Port: {}", port);
// }




// Working with alot of .env files

// use dotenv::dotenv;
// use serde::Deserialize;


// #[derive(Deserialize,Debug)]
// struct Config{
//     database_url:String,
//     api_key:String,
//     port:u16,
// }

// fn main(){
//     dotenv().ok();

//     let config = envy::from_env::<Config>().expect("Failed to load config");
//     println!("{:?}",config);
// }


// Handling .env files in Multiple Environments
// .env -> Default
// .env.development -> development
// .env.production -> production

use dotenv::from_filename; // Use from_filename to load a specific .env file
use std::env;

fn main() {
    // Load a custom .env file
    from_filename(".env.development").ok();

    // Access environment variable
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
    println!("DB URL: {}", db_url);
}



// dotenvy
// use dotenvy::dotenv;
// use std::env;

// fn main() {
//     dotenv().ok();
//     println!("PORT: {}", env::var("PORT").unwrap());
// }
