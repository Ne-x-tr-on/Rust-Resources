//They are Tuples Arrays slices Strings (slice strings)

//Arrays
fn main(){
 arrays();
 tuples();
 slices();
}



//Array are simply a list numbers words etc
fn arrays(){
  let array: [i32; 10] = [12,23,5,45,1,3,2,6,7,81];
  println!("This is an array {:?} of numbers",array);

  //Array of Strings
  let fruits: [&str; 4] = ["apple", "banana", "cherry", "date"];
  //All elements
  println!("Fruits Array: {:?}",fruits);

  //First Element
    println!("First Array: {:?}",fruits[0]);
}



//Tuples are
fn tuples(){
  //tuples
  let human:(String,i32,bool) = ("Alice".to_string(),30,false);
  println!("{:?} is {:?} years old and is {:?}",human.0, human.1, human);

  let mix_tuple = (1,2,3,4,5,[1,2,3,4,5]);
  println!("{:?}",mix_tuple);

  let human2= ("Alice",30,false);
  println!("{:?} is {:?} years old and is {:?}",human2.0, human2.1, human2);
}

//Slices
fn slices(){
  //number Slice
  let numbers:&[i32] = &[1,2,3,4,5,6];
  println!("Number Slice:{:?}",numbers);

  //String Slice
  let animal:&[&str] = &["Lion","Giraffe","Zebra"];
  println!("These are Animals: {:?}",animal);
  

  let animals:&[&String] = &[&"Lion".to_string(),&"Giraffe".to_string(),&"Zebra".to_string()];
  println!("These are Animals: {:?}",animals);

}