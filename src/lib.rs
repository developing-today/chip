#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]
mod app;
pub mod lang;
pub fn main() {
    app::new();
}
