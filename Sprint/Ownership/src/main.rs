pub mod ownership;
pub mod own_function;

#[allow(unused)]
use crate::ownership::ownership;

#[allow(unused)]
use crate::own_function::owner;

#[allow(unused)]
use crate::own_function::take_ownership;

#[allow(unused)]
use crate::own_function::take_return_ownership;

// Each value has one owner
// Only one owner at a time
// When the owner goes out of scope, the value is dropped
fn main() {
    ownership::ownership();
    // own_function::take_ownership();
    own_function::owner();

    // own_function::take_return_ownership("Returns a value".to_string());
}
