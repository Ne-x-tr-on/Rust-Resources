//They are Tuples Arrays

//Arrays
fn main(){
 arrays();
 tuples();
}

fn arrays(){
  let array: [i32; 10] = [12,23,5,45,1,3,2,6,7,81];
  println!("This is an array {:?} of numbers",array);
  let fruits: [&str; 4] = ["apple", "banana", "cherry", "date"];
  //All elements
  println!("Fruits Array: {:?}",fruits);
  //First Element
  
  println!("First Array: {:?}",fruits[0]);
}
fn tuples(){
  //tuples
  let human:(String,i32,bool) = ("Alice".to_string(),30,false);
  println!("{:?} is {:?} years old and is {:?}",human.0, human.1, human);

  let human2= ("Alice",30,false);
  println!("{:?} is {:?} years old and is {:?}",human2.0, human2.1, human2);
}