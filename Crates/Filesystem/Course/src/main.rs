pub mod fs_crud;
use crate::fs_crud::{create_dir,create_file,remove_dir};
fn main(){
    println!("CRUD OPERATION IN fILE SYSTEM");

    remove_dir();
    create_dir();
    create_file();
}