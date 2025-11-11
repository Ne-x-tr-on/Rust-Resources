pub mod fs_crud;
use crate::fs_crud::create_dir;
use crate::fs_crud::create_file;
use crate::fs_crud::remove_dir;
fn main(){
    println!("CRUD OPERATION IN fILE SYSTEM");
    create_dir();
    create_file();
    remove_dir();
}