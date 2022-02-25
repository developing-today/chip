#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]

#[macro_use]
extern crate num_derive;

pub fn main() -> ! {
    chiploxide::lang::repl::token::new();
}
