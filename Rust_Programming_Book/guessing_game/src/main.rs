// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Welcome to the guessing game!");

//    let secret_number = rand::thread_rng().gen_range(1..=10);

//     println!("The secret number is: {}", secret_number);

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read input");

//     println!("You guessed: {}",guess.trim());

//       let guess: u32 = guess
//         .trim()
//         .parse()
//         .expect("Please type a number");

//     match guess.cmp(&secret_number){
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too Big!"),
//         Ordering::Equal => println!("You Win!"),

//     }
// }




// // use std::io;
// // fn  main(){
// //     println!("Welcome");
// //     let mut number = String::new();
// //     println!("Guess a number");

// //     io::stdin()
// //     .read_line(&mut number)
// //     .expect("Error fetching Number");

// //     println!("You guessed {}",number)

// // }


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
     println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {}", secret_number);

    loop{
        let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

        // let guess :i32 = guess.trim().parse().expect("It should be an integer");
        let guess :i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too much"),
        Ordering::Equal =>{
             println!("You win");
            break;
         }
        }
    }
 }












