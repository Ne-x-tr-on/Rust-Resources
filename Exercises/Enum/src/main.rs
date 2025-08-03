// #![warn(dead_code)]
// fn main(){
//     enum IpAddrKind{
//         V4,
//         V6,
//     }

//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;

//     // fn route(ip_kind:IpAddrKind){}

//     // route(IpAddrKind::V4)


//     struct IpAddr{
//         kind:IpAddrKind,
//         address:String,
//     }

//     let _home = IpAddr{
//         kind:IpAddrKind::V4,
//         address:String::from("127.0.0.1")
//     };

    
// }


// fn main(){
//     #[derive(Debug)]
//     enum IpAddrKind {
//     V4,
//     V6,    
// }

// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;
// println!("{}",four);

// fn route(_ip_addr_kind:IpAddrKind){}

// // route(IpAddrKind::v4);
// // route(IpAddrKind::v6);

// // struct IpAddr{
// //     kind:IpAddrKind,
// //     address:String,
// // }

// // let home = IpAddr{
// //     kind:IpAddrKind::V4,
// //     address:String::from("198:0:0:1")
    
// // };
// // println!("{}",four);


// }


// enum Message{
//     Quit,
//     Move {x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }

// struct QuitMessage;
// struct MoveMesage {
//     x:i32,
//     y:i32
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32,i32,i32);

// impl Message{
//     fn call(&self){

//     }
// }

// fn main(){
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// enum Option<T>{
//     None,
//     Some(T),
// }

// fn main(){
//     let _number = Some(5);
//     let _char = Some('e');
//     let _no_number:Option<i32> = None;


//     let _x: Option<i32> = Some(5);
//     let _y:Option<i32> = Some(5);

//     let _sum = _x + _y;
//     println!("{}",_sum);


// }



// #[derive(Debug)]
// enum UsState{
//     Alaska,
//     Alabama,
// }

//  enum Coin{
//     Nickel,
//     Penny,
//     // Dime,
//     Quarter(UsState),
//  }

//  pub fn match_coins(coin:Coin) -> u8{
//     match coin {
//         Coin::Nickel => 5,
//         Coin::Penny => 1,
//         Coin::Quarter(state) => {
//             println!("State is from {:?}",state);
//             25
//         }

//     }
//  }

//  fn main(){
//     let coin = Coin::Quarter(UsState::Alabama);
//     let value = match_coins(coin);
//     println!("Coin value is {:?}",value);
//  }


// practice.rs

// // ========== Exercise 1 ==========
// // Define an enum Coin with Penny, Nickel, Dime, Quarter.
// // Write a function that returns its value in cents.

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// 
// // fn coin_value(coin: Coin) -> u8 {
// //     match ...
// // }


// // ========== Exercise 2 ==========
// // Create enum UsState with 3 states and attach it to Quarter variant.
// // Print the state when the coin is a Quarter.

// #[derive(Debug)]
// enum UsState {
//     California,
//     Texas,
//     Nairobi, // Customized to your interest
// }

// enum CoinWithState {
//     Penny,
//     Nickel,
//     Quarter(UsState),
// }

// // fn coin_value_with_state(coin: CoinWithState) -> u8 {
// //     match ...
// // }


// // ========== Exercise 3 ==========
// // Define enum Message with variants: Quit, Move, Write(String), ChangeColor(r,g,b).
// // Write an impl block with a `call` method to print what kind of message it is.

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// // impl Message {
// //     fn call(&self) {
// //         match self {
// //             ...
// //         }
// //     }
// // }


// // ========== Exercise 4 ==========
// // Use Option<i32> to implement a function that adds two numbers,
// // but only if both are Some values.

// fn add_options(x: Option<i32>, y: Option<i32>) -> Option<i32> {
//     // match or if let ...
//     None
// }


// // ========== Exercise 5 ==========
// // Create an enum IpAddr with V4(String) and V6(String).
// // Write a function that prints the kind and value of the IP.

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// // fn print_ip(ip: IpAddr) {}


// // ========== Exercise 6 ==========
// // Define an enum called `City` with variants Nairobi, Mombasa, Kisumu.
// // Write a function that returns population estimate using match.

// enum City {
//     Nairobi,
//     Mombasa,
//     Kisumu,
// }

// // fn get_population(city: City) -> u32 {}


// // ========== Exercise 7 ==========
// // Create a VendingItem enum with: Soda, Water, Chips, and Price(u32)
// // Write a function that prints item and returns price.

// #[derive(Debug)]
// enum VendingItem {
//     Soda,
//     Water,
//     Chips,
//     Custom(String, u32), // Example: ("Juice", 120)
// }

// // fn vending_price(item: VendingItem) -> u32 {}


// // ========== Exercise 8 ==========
// // Create an enum `Transport` with: Car, Train, Plane.
// // Write a function that prints the average speed for each transport type.

// enum Transport {
//     Car,
//     Train,
//     Plane,
// }

// // fn avg_speed(mode: Transport) -> u32 {}


// // ========== Exercise 9 ==========
// // Create a struct `Person` with name and city. Create a function
// // that prints a greeting based on their city (match on city enum).

// struct Person {
//     name: String,
//     city: City,
// }

// // fn greet(person: Person) {}


// // ========== Exercise 10 ==========
// // Use Option<f32> to write a function that calculates
// // temperature in Celsius from Fahrenheit (if value is Some).

// fn fahrenheit_to_celsius(f: Option<f32>) -> Option<f32> {
//     // Hint: (f - 32.0) * 5.0 / 9.0
//     None
// }

// // =================================
// // To test, write `fn main()` below
// // and call the functions you solve.
// // =================================

// fn main() {
//     // Example test calls (uncomment once implemented):
    
//     // println!("Coin value is: {}", coin_value(Coin::Dime));
    
//     // let coin = CoinWithState::Quarter(UsState::Nairobi);
//     // println!("Coin value is: {}", coin_value_with_state(coin));

//     // let msg = Message::Write(String::from("Hello, Rust!"));
//     // msg.call();

//     // let result = add_options(Some(3), Some(7));
//     // println!("{:?}", result);

//     // print_ip(IpAddr::V4(String::from("192.168.0.1")));

//     // let population = get_population(City::Nairobi);
//     // println!("Population: {}", population);

//     // let item = VendingItem::Custom(String::from("Juice"), 120);
//     // println!("Price: {}", vending_price(item));

//     // println!("Speed: {} km/h", avg_speed(Transport::Train));

//     // let p = Person { name: String::from("Newton"), ci
// }


// practice_questions.rs

// // ========== Exercise 1 ==========
// // Define an enum `Coin` with variants: Penny, Nickel, Dime, Quarter.
// // Write a function `coin_value` that returns the coin’s value in cents.
// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Africa),
// }

// fn coin_value(coin:Coin) -> u8 {
//     match coin{
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(country) => {
//             println!("This is from {:?}",country);
//             25
//         }
        
//     }
// }



// // ========== Exercise 2 ==========
// // Define an enum `UsState` with 3 U.S. states (or include Nairobi for fun).
// // Attach a UsState to the `Quarter` variant of `CoinWithState` enum.
// // Write a function that returns the value of the coin and prints the state if it’s a Quarter.
// #[derive(Debug)]
// enum Africa{
//     Kenya,
//     Egypt,
//     Madagascar,
// }

// fn main(){
//     let _value = coin_value(Coin::Dime);
//     println!("{:?}:{:?}",Coin::Dime,_value);
// }

// ========== Exercise 3 ==========
// Define an enum `Message` with variants:
// - Quit
// - Move { x, y }
// - Write(String)
// - ChangeColor(i32, i32, i32)
// Write a method `call()` inside an `impl` block that prints what the message is.

// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),

// }
// impl Message{
//     fn call(&self){
//       self.Write("Hello i am home".to_string());
//     }
// }




// ========== Exercise 4 ==========
// Use `Option<i32>` to write a function that adds two numbers only if both are Some.
// If either is None, return None.

// ========== Exercise 4 ==========
// Use `Option<i32>` to write a function that adds two numbers only if both are Some.
// If either is None, return None.

use core::option::Option::Some;

fn main(){
    let x: Option<i32> = Some(11);
    let y: Option<i32> = Some(11);

    let results = add(x,y);
    match results{
        Some(val)=> println!("Sum: {}",val),
        None => println!("Error handled Gracefully"),
    }



}

fn add(a:Option<i32>,b:Option<i32>) -> Option<i32> {
    match(a,b){
        (Some(x),Some(y)) => Some(x+y),
        _ => None,
    }
}


// fn main(){
//     let _array:[i32; 3] = [0;3];
//     for item in _array.iter().enumerate(){
//         let (i,x):(usize,&i32) = item;
//         println!("array [{i}] = {x}");
//     }

//     for item in _array.into_iter().enumerate(){
//         let (i,x):(usize,i32) =item;
//         println!("array [{i}] = {x}");

//     }
// }


// ========== Exercise 5 ==========
// Define an enum `IpAddr` with variants:
// - V4(String)
// - V6(String)
// Write a function that prints whether the IP is IPv4 or IPv6, and displays the address.


// ========== Exercise 6 ==========
// Define an enum `City` with variants Nairobi, Mombasa, and Kisumu.
// Write a function that returns a rough population estimate using `match`.


// ========== Exercise 7 ==========
// Define an enum `VendingItem` with variants:
// - Soda
// - Water
// - Chips
// - Custom(String, u32)  // Example: Custom("Juice", 120)
// Write a function that returns the price of the item and prints its name.


// ========== Exercise 8 ==========
// Define an enum `Transport` with variants: Car, Train, Plane.
// Write a function that returns the average speed of each transport in km/h.


// ========== Exercise 9 ==========
// Define a struct `Person` with fields:
// - name: String
// - city: City (from exercise 6)
// Write a function that prints a personalized greeting based on the city they live in.


// ========== Exercise 10 ==========
// Use `Option<f32>` to write a function that converts Fahrenheit to Celsius.
// If the input is `None`, return `None`.
// Formula: (f - 32.0) * 5.0 / 9.0


// ========== Bonus (Optional Main Function) ==========
// Write a `main` function to test all your answers step by step.
