#[allow(unused)]
pub fn test_if(){

  let age_to_drive = 16u8;
  println!("Please Enter Your Age");

  let mut myinput = String::new();
  std::io::stdin().read_line(&mut myinput).expect("Failed to read line");

  match myinput.trim().parse::<u8>(){
    Ok(age) => {
      if age >=85 {
        println!("Renew your Licence it might be your last one");
        } else if age >= age_to_drive{
        println!("You are eligible to drive");
         }else {
        let years = age_to_drive - age;
        println!("You will be eligible to drive in {:?} years",years);
      }
    }
    Err(e)=> println!("Please Enter a valid number"),
  }

}

// #[allow(unused)]

// pub fn test_if(){
//   let age_to_drive = 16u8;
//   println!("Please Enter your age");

//   // let myinput = &mut String::from("");
//   // std::io::stdin().read_line(myinput).unwrap();
//   // let age = myinput.trim().parse::<u8>().unwrap();

//   let myinput = &mut String::from("");
//   std::io::stdin().read_line(myinput).unwrap();

//   let age = myinput.trim().parse::<u8>().unwrap();

//   if age >= age_to_drive{
//     println!("You are eligible to drive");
//   }
//   else{
//     let years = age_to_drive - age;
//     println!("Wait for {:?} years",years);
//   }

// }

pub fn test_while(){

  let age_to_drive = 16u8;
  let mut current_age = 0u8;

  while current_age < age_to_drive{
    current_age += 10;
    println!("Waiting ...");
  }
}


pub fn test_loop(){
  let mut x = 1;
  loop{
    println!("Hello from Rust");
    if x > 10{
      break;
    }
    x += 1;
  }
}

pub fn test_forloop(){
  let elements :[i32;10] = [12,23,34,45,13,14,15,16,17,18];
  let age_to_drive = 16i32;

//   for  values in elements{
//     if values >= age_to_drive{
//       println!("You are eligible to drive");
//     } else {
//       println!("You are not eligible to drive");
//     }
//   }


for value in elements.iter() {
    if *value >= age_to_drive {
        println!("Age {}: Eligible", value);
    } else {
        println!("Age {}: Not eligible", value);
    }
}

}
