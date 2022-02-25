#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]

use chiploxide::lang;

#[macro_use]
extern crate num_derive;

pub fn main() -> () {
    lang::repl::token::new("123123");
}
