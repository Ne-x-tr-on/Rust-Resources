use serde::{Serialize, Deserialize};
use tokio;
use std::error::Error;
use uuid::Uuid;

// 1. Define the FounderProfile struct here
// #[...]
// struct FounderProfile { ... }
#[derive(Debug,Serialize,Deserialize)]
struct FounderProfile{
    founder_id:Uuid,
    platform_role:String,
    projects_started:u32,
}


// 2. Implement the asynchronous function here
// async fn save_profile_to_file(...) -> Result<(), Box<dyn Error>> { ... }
async fn save_profile_to_file(profile:FounderProfile)->Result<(),Box<dyn Error>>{
    let serialization = serde_json::to_string_pretty(&profile)?;

    tokio::fs::write("davion_profile.json",serialization).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 6. Create a sample profile
    let new_founder = FounderProfile {
        founder_id: Uuid::new_v4(), // Generate a unique ID
        platform_role: "Pioneer".to_string(),
        projects_started: 5,
    };

    println!("Attempting to save profile with ID: {}", new_founder.founder_id);
    println!("Profile Role: {}", new_founder.platform_role);

    // 6. Call the asynchronous function
    match save_profile_to_file(new_founder).await {
        Ok(_) => println!("✅ Profile successfully saved to davion_profile.json"),
        Err(e) => eprintln!("❌ Failed to save profile: {}", e),
    }

    // --- Optional: Verify the file content ---
    match tokio::fs::read_to_string("davion_profile.json").await {
        Ok(content) => {
            println!("\n--- Content of davion_profile.json ---");
            println!("{}", content);
        },
        Err(e) => eprintln!("\nCould not read file for verification: {}", e),
    }
    
    Ok(())
}