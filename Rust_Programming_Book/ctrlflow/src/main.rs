// fn main(){
//     let number = 3;

//     if number < 5{
//         println!("Too low");
//     } else if number > 5 {
//         println!("Too high");
//     }
// }

// fn main(){
//     let number = 20;

//     if number % 4 == 0{
//         println!("No div by 4");
//     } else if number % 3 == 0 {
//         println!("No divisible by 3");
//     } else if number % 2 == 0 {
//         println!("No diviible by 2");
//     }
// }

// fn main() {
//  let condition = false;
//  let number = if condition { 5 } else { 6 };
//  println!("The value of number is: {number}");
// }


// fn main(){
//     let mut counter = 0;

//     let result = loop{
//         counter += 1;

//         if counter == 10{
//             break counter/10;
//         }
//      };
     
//      println!("The result is {result}");
// }


// fn main(){
//     let mut number = 50;

//     while number != 0 {
//         println!("{number}");

//         number -=1;
        
//     }

//     println!("Done");
// }

// Looping through a collection using While

fn main(){
    // let a = [10,20,30,40,50];

    // Using While loop

    // let mut index = 0;

    // while index < 4 {
    //     println!("the value is {}",a[index]);
    //     index+=1;
    // }

    // Using a for loop
    // for data in a {
    //     println!("The values are {data}");
    // }


for number in (1..20).rev() {
    println!("{number}!");
 }
     println!("LIFTOFF!!!");
}