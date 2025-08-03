// 1. Write a function `hello()` that prints 'Hello, Rust!'
fn hello(){
  println!("Hello, Rust!");
}
// 2. Write a function that takes a name and prints 'Hello, <name>!'
fn tkname(name:&str){
  println!("Hello Mr, {}!", name);
}
// 3. Create a function that returns the square of a number.
fn square(x: i32) -> i32{
  x*x
}
// 4. What's the difference between a statement and an expression? Give examples.
//An expression have a return type while a statement doesnt

// 5. Create a tuple with your name, age, and height. Print all values.
fn me(){
  let me_be:(String,i32,f32)=("Nextron".to_string(),19,1.8);
  println!("{:?}",me_be);
  }
// 6. Destructure a tuple into separate variables and print them.
fn medestrucured(){
  let me_be:(String,i32,f32)=("Nextron".to_string(),19,1.8);
  println!("My name is {}.I am {} years old and my height is {} meters",me_be.0,me_be.1,me_be.2);
  }
// 7. Create an array of 5 numbers. Print the first and last element.
fn array(){
  let arrayofno:[i32;5]= [1,2,3,4,5];
  println!("The first no is {} and the last no is {}",arrayofno[0],arrayofno[4]);
  // 8. Use a loop to print each element of an array.
  for element in arrayofno.iter(){
      println!("{}",element);
  }
}
// 9. Create a slice of an array containing the middle three values.
fn exerslice(){
  let numbers = [1, 2, 3, 4, 5, 6, 7];
  let slice = &numbers[1..4];
  println!("--The middle values are as below--");
  for value in slice.iter(){
    println!("{}",value);
  }
}
// 10. Write a function that takes a slice and prints all its elements.
fn print_slice(sliceval:&[i32]){
  for value in sliceval.iter(){
    println!("{}",value);
  }
}
// 11. Create a mutable String and append another word using `.push_str()`.
fn mtstring(){
  let mut greet = String::from("Hello ");
  greet.push_str("David");
  println!("{} ",greet);
}
// 16. Create an array of strings (5 fruits) and print the 3rd one.
fn fruits(){
  let fruit [&str,5] = ["Apple","Banana","Cherry","Date","Elderberry"];
  println!("{}",fruit);
}
// 17. Create a tuple of (i32, f32, &str) and print each with formatting.
// 18. Write a function that calculates the average of 3 i32 values.
// 19. Create a function that returns a greeting as a `String`.
// 20. Print a formatted message using tuple values (like student data).
// 21. Create a function that returns the length of a string.
// 22. Print the length of an array using `.len()`.
// 23. Use a for loop to calculate and print the sum of all array items.
// 24. Return a string slice from a given string.
// 25. Write a function that takes a name and score, and prints 'Congrats <name>, you scored <score>'.

fn main(){
  println!("Question 1");
  hello();

  println!("Question 2");
  tkname("Nexron");

  println!("Question 3");
  println!("The square of 5 is: {}",square(5));

  println!("Question 4");
  me();
  
  println!("Question 5");
  medestrucured();

  println!("Question 6");
  array();

  println!("Question 7");
  exerslice();

  println!("Question 8");
  let slicevalue = [1,2,3,4,5,6,7,8];
  print_slice(&slicevalue);
  println!("Break");
  let sliceval = &slicevalue[1..5];
  print_slice(sliceval);

  println!("Question 9");
  mtstring();
}