#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]

#[macro_use]
extern crate num_derive; //compiles without this? possibly built-in now? 2022-02-02 ish update

use chiploxide::lang::repl::token::repl;

pub fn main() -> ! {
    repl();
}
