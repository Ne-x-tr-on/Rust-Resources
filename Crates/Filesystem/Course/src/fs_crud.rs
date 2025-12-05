use std::fs;
use std::path::Path;
use std::fs::remove_dir_all;

pub fn remove_dir(){
  let path = "./data";
  _ = remove_dir_all(path);
}

pub fn create_dir(){
  let path = "./data";
  let my_path = Path::new(path);

  if my_path.exists(){
    println!("Path: \n {} \nalready exist",path);
    return;
  }

  let create_dir = fs::create_dir("./data");
  if create_dir.is_ok(){
    println!("Directory Created");
  }
  else{
    eprintln!("Failed to create the Directory")
  }
}

pub fn create_file(){
  let path = "./data/file.txt";
  let contents = "Johnte";
  _ = std::fs::write(path, contents)
}