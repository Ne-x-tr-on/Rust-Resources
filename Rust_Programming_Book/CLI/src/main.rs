use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::Write};
use clap::{Parser, Subcommand};

#[derive(Serialize, Deserialize, Debug)]
struct Concept {
    name: String,
    review_date: Option<String>,
    rustbook: bool,
    exercises_done: bool,
    confidence: u8, // 1 to 5 scale
}

#[derive(Parser)]
#[command(name = "Rust Learning Tracker")]
#[command(version = "1.0")]
#[command(about = "Track your Rust concepts")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new concept
    Add {
        name: String,
    },
    /// Update an existing concept
    Update {
        name: String,
        #[arg(short = 'd', long)] // d for review_date
        review: Option<String>,
        #[arg(short = 'b', long)] // b for rustbook
        rustbook: Option<bool>,
        #[arg(short = 'e', long)] // e for exercises
        exercises: Option<bool>,
        #[arg(short = 'c', long)] // c for confidence
        confidence: Option<u8>,
    },
    /// List all concepts
    List,
}

fn main() {
    let cli = Cli::parse();
    let file_path = "data/progress.json";

    // Load existing concepts or initialize empty list
    let mut concepts: Vec<Concept> = fs::read_to_string(file_path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(Vec::new);

    match &cli.command {
        Commands::Add { name } => {
            if concepts.iter().any(|c| c.name == *name) {
                println!("⚠️  Concept '{}' already exists.", name);
            } else {
                concepts.push(Concept {
                    name: name.to_string(),
                    review_date: None,
                    rustbook: false,
                    exercises_done: false,
                    confidence: 0,
                });
                println!("✅ Added concept: {}", name);
            }
        }
        Commands::Update {
            name,
            review,
            rustbook,
            exercises,
            confidence,
        } => {
            let mut found = false;
            for c in &mut concepts {
                if &c.name == name {
                    if let Some(date) = review {
                        c.review_date = Some(date.clone());
                    }
                    if let Some(rb) = rustbook {
                        c.rustbook = *rb;
                    }
                    if let Some(ex) = exercises {
                        c.exercises_done = *ex;
                    }
                    if let Some(conf) = confidence {
                        c.confidence = *conf;
                    }
                    found = true;
                    println!("✅ Updated concept: {}", name);
                }
            }
            if !found {
                println!("❌ Concept '{}' not found.", name);
            }
        }
        Commands::List => {
            if concepts.is_empty() {
                println!("📂 No concepts tracked yet.");
            } else {
                for c in &concepts {
                    println!(
                        "📘 {} | Reviewed: {:?} | RustBook: {} | Exercises: {} | Confidence: {}",
                        c.name, c.review_date, c.rustbook, c.exercises_done, c.confidence
                    );
                }
            }
        }
    }

    // Save changes
    let json = serde_json::to_string_pretty(&concepts).expect("❌ Failed to serialize data");
    fs::create_dir_all("data").unwrap();
    let mut file = File::create(file_path).expect("❌ Failed to create progress file");
    file.write_all(json.as_bytes())
        .expect("❌ Failed to write progress data");
}
