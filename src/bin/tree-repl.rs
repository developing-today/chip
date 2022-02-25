#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]

use chiploxide::lang::repl::tree::repl;

#[macro_use]
extern crate num_derive;

pub fn main() -> ! {
    repl();
}
