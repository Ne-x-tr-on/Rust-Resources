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
#[allow(unused)]
pub fn test_if(){

  let age_to_drive = 16u8;
  println!("Please Enter Your Age");

  let mut myinput = String::new();
  std::io::stdin().read_line(&mut myinput).expect("Failed to read line");

  match myinput.trim().parse::<u8>(){
    Ok(age) => {
      if age >= age_to_drive{
        println!("You are eligible to drive");
      } else {
        let years = age_to_drive - age;
        println!("You will be eligible to drive in {:?} years",years);
      }
    }
    Err(e)=> println!("Please Enter a valid number"),
  }

}