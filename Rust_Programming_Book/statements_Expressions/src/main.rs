// Function bodies are made up of a series of statements optionally ending in an expression.


// Statements are instructions that perform some action and do not return a value.

// Expressions evaluate to a resultant value.


// fn main(){
//     let y = {
//         let x = 3;
//         x+1
//     };
//     println!("{y}");
// }

// Functions with Return Values

// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow ( -> ).

// fn main(){
//     let _x = five();
//     println!("{_x}");
// }

// fn five() -> i32{
//     5
// }


fn main() {
 let x = plus_one(5);
 println!("The value of x is: {x}");
}
fn plus_one(x: i32) -> i32 {
 x + 1
}
