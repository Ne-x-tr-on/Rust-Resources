use std::fs;
// use std::fs::remove_dir_all;

pub mod fs_dir;
use crate::fs_dir::create_dir;
fn main(){
    create_dir();
}