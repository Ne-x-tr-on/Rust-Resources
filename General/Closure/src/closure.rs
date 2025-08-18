pub fn closures(){
  // let add = |x:i8, y:i8| x+y;
  // let result = add(-2,8);
  // println!("{}",result);
  
  let add = |x:i32,y:i32| {
    println!("Value for x :{} Value for y :{}",x,y);
    x+y
  };
  let results = add(3,9);
  println!("The Result is {}",results);
  
  #[derive(Debug)]
   struct Person{
    first_name:String,
    last_name:String,
  }

   let mut p1 = Person{
    first_name:"Newton".to_string(),
    last_name:"Kamau".to_string(),
  };

  let mut change_name = || p1.last_name = "David".to_string();
  change_name();
  println!("New name is: {:?}",p1.last_name);

}