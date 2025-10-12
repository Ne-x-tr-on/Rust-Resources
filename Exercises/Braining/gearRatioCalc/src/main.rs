use std::io;

fn main() {
    println!("=== Gear Ratio Calculator ===");

    // Ask for the number of stages
    println!("Enter number of gear stages:");
    let mut stages_input = String::new();
    io::stdin().read_line(&mut stages_input).unwrap();
    let stages: usize = stages_input.trim().parse().unwrap();

    let mut overall_ratio = 1.0;

    for i in 1..=stages {
        println!("\nStage {}:", i);

        // Driver gear teeth
        println!("Enter number of teeth on driver gear:");
        let mut driver_input = String::new();
        io::stdin().read_line(&mut driver_input).unwrap();
        let driver_teeth: f64 = driver_input.trim().parse().unwrap();

        // Driven gear teeth
        println!("Enter number of teeth on driven gear:");
        let mut driven_input = String::new();
        io::stdin().read_line(&mut driven_input).unwrap();
        let driven_teeth: f64 = driven_input.trim().parse().unwrap();

        let stage_ratio = driven_teeth / driver_teeth;
        println!("Stage {} Gear Ratio = {:.2}", i, stage_ratio);

        overall_ratio *= stage_ratio;
    }

    println!("\nOverall Gear Ratio = {:.2}", overall_ratio);
}
