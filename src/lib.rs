#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]
extern crate num_traits;
mod app;
mod lang;
pub fn main() {
    lang::main();
    app::new();
}
