pub fn test_vec_int(){
  let mut my_ints :Vec<i32> = Vec::new();

  my_ints.push(30);
  my_ints.push(30);
  my_ints.push(30);
  my_ints.push(30);
  my_ints.push(30);
  my_ints.push(30);

 

  println!("The Data: {:?}",my_ints);
  println!("Length of the Vec: {:?}",my_ints.len());
  println!("Capacity: {:?}",my_ints.capacity());

   println!("First Item in The Vec is {:?}",my_ints [0]);
   println!("All items in The Vec is {:?}",&my_ints.as_slice()[0..2]);
}
  
pub fn vec_string(){
  let names = vec!["Trevor","Arnold","Billie","BroCode","Samson"];
  
  for name in &names {
    println!("Processing {}",name);
  }

  println!("{:?}",&names)
}
struct Car {
  manufactre:String,
  model:String,
}
 
pub fn test_vec_car(){
  let car_list = vec![Car{manufactre:"Porshe".to_string(),model:"Panamera".to_string()};10];
  println!("{:?}",car_list);
}