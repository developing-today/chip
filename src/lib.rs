#![feature(fn_traits)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unwrap_infallible)]
#![feature(stmt_expr_attributes)]
mod app;
pub mod lang;
pub fn main() {
    app::new();
}
