use std::io;

fn main(){
    println!("Welcome to my game");
    let mut name = String::new();
    println!("What is your name: ");
    
    io::stdin().read_line(&mut name).expect("Error Reading name");
    let name = name.trim();
  

    println!("Hey {name} welcome to learnza");
}