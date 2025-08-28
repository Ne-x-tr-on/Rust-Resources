pub fn test_iterators(){
let fruit_list = vec!["Strawberry","BlueBerry","Mango","Orange","Apple"];

let cars = vec!["Toyota", "Honda", "BMW", "Mercedes", "Ford"];

let mut fruit_iter = fruit_list.iter();

let mut my_cars = cars.iter();






// for loop_fruit in fruit_iter{
//   println!("{:?}",loop_fruit);
// }

// nextMethod
fruit_iter.next();
let item01 = fruit_iter.next();
println!("First item in Iterator is : {:?}",item01.unwrap());

// Chain Method
let mixed = fruit_list.iter().chain(my_cars);

 let all_data:Vec<&&str> = mixed.clone().collect();
 println!("{:?}",all_data);

// for data in mixed
// {
//   println!("{:?}",data);
// }

}