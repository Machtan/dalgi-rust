#[cfg(feature = "rsdl2-support")]
extern crate rsdl2;

mod input_key;
mod notification;

#[macro_use]
pub mod input;

#[cfg(feature = "rsdl2-support")]
mod rsdl2_input;

#[test]
fn it_works() {}
