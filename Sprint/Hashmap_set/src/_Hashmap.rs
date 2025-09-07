use std::collections::HashMap;

pub fn test_maps(){
  let mut name_age:HashMap<String,i32> = HashMap::new();

  name_age.insert("Newton".to_string(),19);

  println!("List of names:\n{:?} ",name_age);
  // println!("List of names:\n{:?} ",name_age.unwrap());
  
}