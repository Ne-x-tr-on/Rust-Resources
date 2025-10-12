use std::io;

fn main(){
  println!("---->| Welcome to Gear Ration Calculator |<----");

  println!("Enter the number of Stages");
  let mut stages_input = String::new();
  io::stdin().read_line(&mut stages_input).unwrap();
  let stages:usize = stages_input.trim().parse().unwrap();

  let overall_ratio = 1.0;

  for i in 1..=stages{
    println!("Hello");
  }
}