// ===========================
// RUST MODULES PRACTICE QUESTIONS
// ===========================

// Q1. Create a module named `math_utils` and define a function `add` that returns the sum of two i32 numbers.

// Q2. Move the `add` function to a separate file called `math_utils.rs` and use `mod` to import it in `main.rs`.

// Q3. Define a public struct `Config` inside a module `settings` with fields `theme: String` and `volume: u8`.

// Q4. Inside a module `network`, define a private function `connect` and a public function `start` that calls `connect`.

// Q5. Create a nested module `app::auth` and define a function `login`.

// Q6. Use the `use` keyword to bring `login` into scope from `app::auth`.

// Q7. Write a module `logger` with a public function `log_info(msg: &str)` that prints a message prefixed with "[INFO]".

// Q8. Demonstrate how to re-export `log_info` from `logger` as `info` using `pub use`.

// Q9. Use `super::` to access a function from the parent module.

// Q10. Create a module `math` with submodules `add` and `multiply`, each having a function with the same name.

use std::fs;
use std::io::{self, Write};
use rand::seq::SliceRandom;

fn main() {
    let topic = "mod"; // Or let user choose from command-line args
    // Define paths for question and answer files based on the topic
    let question_file = format!("src/questions/{}_questions.rs", topic);
    let answer_file = format!("src/answers/{}_answers.rs", topic);

    // Read questions from the specified file.
    // .expect() will cause the program to panic if the file cannot be read.
    let questions = fs::read_to_string(&question_file)
        .expect("Failed to read question file");

    // Read answers from the specified file.
    // .expect() will cause the program to panic if the file cannot be read.
    let answers = fs::read_to_string(&answer_file)
        .expect("Failed to read answer file");

    // Filter lines that start with "// Q" to identify questions
    let question_lines: Vec<_> = questions.lines()
        .filter(|line| line.trim().starts_with("// Q"))
        .collect();

    // Initialize a random number generator
    let mut rng = rand::thread_rng();
    // Choose a random question from the filtered lines
    if let Some(random_q) = question_lines.choose(&mut rng) {
        println!("💡 Question:\n{}", random_q);

        // Prompt the user for their answer
        print!("Your answer (press Enter to skip): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap(); // Read user input

        // Check if the user skipped the question
        if user_input.trim().is_empty() {
            // If skipped, try to find and show the matching answer
            // This logic assumes question numbers are consistent (e.g., "// Q1. Some question")
            if let Some(q_number_part) = random_q.split('.').next() {
                // Extract the question number (e.g., "// Q1")
                let q_number = q_number_part.trim();

                // Find the answer block by skipping lines until the question number is found,
                // then taking lines until the next question marker "// Q" is encountered.
                let answer_lines: Vec<_> = answers.lines()
                    .skip_while(|line| !line.trim().starts_with(q_number)) // Skip until the line containing the question number
                    .skip(1) // Skip the question line itself
                    .take_while(|line| !line.trim().starts_with("// Q")) // Take lines until the next question
                    .collect();
                
                // Print the collected answer lines
                println!("\n✅ Correct Answer:\n{}\n", answer_lines.join("\n"));
            }
        } else {
            // If the user provided an answer, just acknowledge it
            println!("👍 Great! You attempted it.");
        }
    } else {
        println!("No questions found for topic '{}' or question file is empty.", topic);
    }
}

