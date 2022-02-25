pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!(
        "{:?}",
        token::new(
            r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!(
        "{:?}",
        token::new(
            r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#"pub mod trees;
use chiploxide::lang::repl::token;
fn main() {
    println!("{:?}", token::new(r#""#
        )
    );
}
/*
maybe if i keep iterating i will make a file that prints itself
*/
