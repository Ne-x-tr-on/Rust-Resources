
fn main() {
//How we print in Rust
    println!("Lets start the journey we all");
//Calling function in main
    ints();
    bool();
    char();
    str();
}

//Working with Int values
fn ints(){
    let x: i32 = -10;
    let y: u64 = 90;
    println!("Signed Integer: {}", x);
    println!("Unusigned Integer: {}", y);
} 

//Working with Strings
fn str(){
    let _myname: &str = "Nextron";
    println!("{}",_myname);
}


//Working with Boolean Values
fn bool(){
    let is_raining: bool = true;
    println!("It is {} raining",is_raining);
} 


//Working with char Values
fn char(){
    let letter: char = 'a';
    println!("So the first letter of the alphabet is {}", letter);  // prints: So the first letter of the alphabet is a
}

