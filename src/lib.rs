#[cfg(feature = "rsdl2_support")]
extern crate rsdl2;

mod input_key;
#[macro_use]
pub mod input;

#[cfg(feature = "rsdl2_support")]
mod rsdl2_input;

#[test]
fn it_works() {}
