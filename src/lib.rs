#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]
extern crate num_traits;
mod app;
pub mod lang;
pub fn main() {
    app::new();
}
