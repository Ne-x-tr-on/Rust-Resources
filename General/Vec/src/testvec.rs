#[allow(unused,dead_code)]

pub fn test_vec_int(){

  let mut my_ints: Vec<i32> = Vec::new();
  // let my_ints = vec![];

  my_ints.push(10);
  my_ints.push(20);
  my_ints.push(30);
  my_ints.push(40);
  my_ints.push(50);

  println!("My Ints: {:?}\nLength:  {:?}",my_ints,my_ints.len());

  println!("First item in Vec is: {:?}",my_ints[4]);

  println!("Total Range: {:?}",my_ints.as_slice());

  println!("Specific Range: {:?}",&(&my_ints).as_slice()[0..2]);

  println!("Element not in scope: {:?}",my_ints.get(10));

}

pub fn test_vec_string(){
   let first_name: Vec<&str> = vec![
        "Alice",
        "Brian",
        "Catherine",
        "David",
        "Elena",
        "Frank",
    ];

    for names in first_name.clone(){
      println!("Processing {} ...",names);
    }
}

#[derive(Debug,Clone)]
struct Car{
  manufacturer:String,
  model:String,
}

pub fn test_vec_car(){
  let car_list = vec![
    Car{
      manufacturer:"Porsche".to_string(),
      model:"Panamera".to_string(),
    }; 10];
    // println!("{:?}",car_list);

    
    // let name = vec!["David";10];     
    // println!("Name: {:?}\n Length: {:?}",name,name.len());

let mut car_list: Vec<Car> = vec![];
let mut car_lot2:Vec<Car> = vec![];

for _ in 1..100u8 {
  car_list.push(Car{
      manufacturer:"Porsche".to_string(),
      model:"Panamera".to_string(),
    });
}
for _ in 1..100u8 {
// Push
  car_lot2.push(Car{
      manufacturer:"Hyundai".to_string(),
      model:"Sonata".to_string(),
    });
}
// Append
  car_list.append(&mut car_lot2);
// Insert
  car_list.insert(
    0, 
    Car { 
      manufacturer: "Lamborgini".to_string(),
      model:"Avenda".to_string()
     });
  //  Remove
  car_list.remove(0);

    let keep = |e:&Car|{if e.manufacturer == "Hyundai"{return true;}else {
      {return  false;}
  }};
  // Retain
  car_list.retain(keep);
  // Reserve Extra Memory
  car_list.reserve(5000);

  println!("{:?}",car_list);
  println!("{:?}",car_list.len());
  println!("{:?}",car_list.capacity());

  println!("{:?}",car_lot2);
  println!("{:?}",car_lot2.len());
  println!("{:?}",car_lot2.capacity());

  println!("{:?}",car_list.get(0).unwrap());

}