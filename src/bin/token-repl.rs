#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]
pub fn main() -> ! {
    chip::lang::repl::token::new();
}
