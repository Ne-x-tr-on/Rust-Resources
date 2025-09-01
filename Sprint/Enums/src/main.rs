pub mod module;
pub mod character;

#[allow(unused)]
use crate::module::test_option;

#[allow(unused)]
fn main(){
    let result = test_option();
    println!("The value: {0}",result.unwrap());
    assert_eq!(result.is_some(),true);
}