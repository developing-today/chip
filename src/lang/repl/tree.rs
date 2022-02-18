// use std::io;
// use std::io::Write;

// use crate::lang::{syntax::trees::Tree, tokens::Token};

// pub fn new(tokens: Vec<Token>) -> Tree {
//     // cli input arguments (clap?)
//     // <binary> <file descriptor> <file descriptor> ...
//     // run each sequentially, stop on first error
//     // <binary> "<string>"
//     // run as if it were a file
//     // <binary>
//     // open repl, parse at each newline,
//     // repl ignores newlines escaped with '\'
//     // todo: implement
//     // todo: flags
//     Tree::new(tokens)
// }

// fn rep(tokens: Vec<Token>) -> Tree {
//     new(tokens)
// }

// pub fn repl() -> ! {
//     std::io::stdout().flush().unwrap();
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("error: unable to read user input");
//     print!("Token> ");
//     loop {
//         print!("Token: {:#?}\nToken> ", rep(super::token::new(&input)));
//     }
// }
