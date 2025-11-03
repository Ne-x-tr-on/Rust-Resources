use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Founder {
    id: Uuid,
    name: String,
    role: String,
    last_updated: String,
}

async fn write_file(filed: &Vec<Founder>) -> Result<(), Box<dyn Error>> {
    println!("📝 Writing founder data into file...");
    let json = serde_json::to_string_pretty(filed)?;
    fs::write("Founder.json", json).await?;
    println!("✅ Data written successfully to Founder.json\n");
    Ok(())
}

async fn read_file() -> Result<Vec<Founder>, Box<dyn Error>> {
    let data = match fs::read_to_string("Founder.json").await {
        Ok(content)=>content,
        Err(_) => {
            println!("No existing file found");
            return Ok(Vec::new());
        }
    };
    let founder: Vec<Founder> = serde_json::from_str(&data)?;
    println!("📖 Read founder data successfully: {:?}", founder);
    Ok(founder)
}

async fn update_role(new_role: &str) -> Result<(), Box<dyn Error>> {
    println!("⚙️ Starting update process...");

    // Step 1: Backup existing file before changes
    if let Err(e) = fs::copy("Founder.json", "Founder_Backup.json").await {
        println!("⚠️ Warning: Could not create backup file ({})", e);
    } else {
        println!("📂 Backup created successfully: Founder_Backup.json");
    }

    // Step 2: Read existing data
    let mut founders = read_file().await?;
    if founders.is_empty() {
        println!("❌ No founders found to update.");
        return Ok(());
    }

    // Step 3: Update role for each founder
    for founder in founders.iter_mut() {
        let old_role = founder.role.clone();
        founder.role = new_role.to_string();
        founder.last_updated = Utc::now().to_string();

        println!("✅ Updated role for {} from \"{}\" ➜ \"{}\"",
                 founder.name, old_role, founder.role);
        println!("🕒 Last updated at: {}", founder.last_updated);
    }

    // Step 4: Save updated data back to file
    let updated_json = serde_json::to_string_pretty(&founders)?;
    fs::write("Founder.json", updated_json).await?;
    println!("💾 All changes saved successfully to Founder.json\n");

    Ok(())
}

async fn add_founder(new_founder: Founder) -> Result<(), Box<dyn Error>> {
    // Step 1: Read existing founders
    let mut other_founders = read_file().await?;
    println!("✅ Accessed existing founders successfully.");

    // Step 2: Check if founder already exists
    if other_founders.iter().any(|f| f.name == new_founder.name) {
        println!("⚠️ Founder \"{}\" already exists — skipping addition.", new_founder.name);
        return Ok(());
    }

    // Step 3: Add new founder
    let mut founder_to_add = new_founder.clone();
    founder_to_add.last_updated = Utc::now().to_string();
    other_founders.push(founder_to_add);

    // Step 4: Serialize and write to file
    let updated_json = serde_json::to_string_pretty(&other_founders)?;
    fs::write("Founder.json", updated_json).await?;

    // Step 5: Log success
    println!(
        "✅ Added new founder successfully: {} ({})",
        new_founder.name, &new_founder.role
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let founders = vec![
        Founder {
            id: Uuid::new_v4(),
            name: "Newton".to_string(),
            role: "Backend_Developer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Shillah".to_string(),
            role: "Frontend_Engineer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Ridhwan".to_string(),
            role: "Mobile_Developer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Melissa".to_string(),
            role: "UI/UX_Designer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Kamau".to_string(),
            role: "AI_Researcher".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Lydia".to_string(),
            role: "Project_Manager".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Neza".to_string(),
            role: "Fullstack_Developer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Derrick".to_string(),
            role: "Cloud_Architect".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Faith".to_string(),
            role: "DevOps_Engineer".to_string(),
            last_updated: Utc::now().to_string(),
        },
        Founder {
            id: Uuid::new_v4(),
            name: "Brian".to_string(),
            role: "Data_Scientist".to_string(),
            last_updated: Utc::now().to_string(),
        },
    ];

    write_file(&founders).await?;
    update_role("AI_Developer").await?;
    Ok(())
}
