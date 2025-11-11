use std::fs;
use std::path::Path;

pub fn create_dir(){
  let path = "./data";
  let my_path = Path::new(path);

  if my_path.exists(){
    println!("Path: \n {} \nalready exist",path);
  }

  let create_dir = fs::create_dir("./data");
  if create_dir.is_ok(){
    println!("Directory Created");
  }
  else{
    eprintln!("Failed to create the Directory")
  }
}

// pub fn dir_path (){

// }