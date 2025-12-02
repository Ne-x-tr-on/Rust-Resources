use std::fs;
use std::path::Path;

fn main() {
    create_dir();
    create_file();
}

pub fn create_dir() {
    let path = "./profile";
    let new_dir = Path::new(path);

    if new_dir.exists() {
        println!("Failed {:?} already exists", &new_dir);
        return;
    }
    let create_new = fs::create_dir(&new_dir);
    if create_new.is_ok() {
        println!("CREATEDIR::OK");
    } else {
        println!("CREATEDIR::ERR");
    }
}

pub fn create_file() {
    let path = "./profile/file.txt";
    let contents = "Let the young man go out and hunt the elephant,\n if He Kills it His poverty Ends,\n If the elephant Kills Him his poverty Ends";
    _ = fs::write(path, contents);
    println!("FILEWRITTEN_OK");
}
