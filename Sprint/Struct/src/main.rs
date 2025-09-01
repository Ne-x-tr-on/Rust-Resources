use std::cell::Cell;
pub mod module;
#[allow(unused)]
use crate::module::test;
#[allow(unused)]
use crate::module::newperson;

fn main() {
    // test();
    module::newperson();
    // println!("{:?}",person1.last_name);
}
