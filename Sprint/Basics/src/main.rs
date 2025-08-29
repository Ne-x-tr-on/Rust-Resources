use crate::compound_data_type::{arrayfn, slicesfn, str_slicefn, tupplesfn};
#[allow(dead_code)]
use crate::primitive_data_type::{boolean, floats, integer};

// Primitive DataTypes
pub mod primitive_data_type;
pub mod compound_data_type;
#[allow(unused)]
use crate::primitive_data_type::charfn;



fn main() {
// Primitive DataTypes
integer();
floats();
boolean();
primitive_data_type::charfn();

// Compound DataTypes
arrayfn();
tupplesfn();
slicesfn();
str_slicefn();

}
