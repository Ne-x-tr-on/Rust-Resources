use std::fs;
use std::io::{self, Write};
use rand::seq::SliceRandom;

fn main() {
    let topic = "mod"; // Or let user choose from command-line args
    let question_file = format!("src/questions/{}_questions.rs", topic);
    let answer_file = format!("src/answers/{}_answers.rs", topic);

    let questions = fs::read_to_string(&question_file)
        .expect("Failed to read question file");

    let answers = fs::read_to_string(&answer_file)
        .expect("Failed to read answer file");

    let question_lines: Vec<_> = questions.lines()
        .filter(|line| line.trim().starts_with("// Q"))
        .collect();

    let mut rng = rand::thread_rng();
    if let Some(random_q) = question_lines.choose(&mut rng) {
        println!("💡 Question:\n{}", random_q);

        print!("Your answer (press Enter to skip): ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim().is_empty() {
            // show the matching answer
            if let Some(q_number) = random_q.split('.').next() {
                let answer_lines: Vec<_> = answers.lines()
                    .skip_while(|line| !line.contains(q_number))
                    .take_while(|line| !line.contains("// Q")) // until next Q
                    .collect();
                println!("\n✅ Correct Answer:\n{}\n", answer_lines.join("\n"));
            }
        } else {
            println!("👍 Great! You attempted it.");
        }
    }
}
