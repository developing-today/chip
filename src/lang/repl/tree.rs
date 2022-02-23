use std::io;
use std::io::Write;

use crate::lang::{syntax::trees::Tree, tokens::Tokens};

pub fn new(tokens: Tokens) -> Tree {
    // cli input arguments (clap?)
    // <binary> <file descriptor> <file descriptor> ...
    // run each sequentially, stop on first error
    // <binary> "<string>"
    // run as if it were a file
    // <binary>
    // open repl, parse at each newline,
    // repl ignores newlines escaped with '\'
    // todo: implement
    // todo: flags
    Tree::new(tokens)
}

fn rep(tokens: Tokens) -> Tree {
    new(tokens)
}

pub fn repl() -> ! {
    std::io::stdout().flush().unwrap();
    print!("Tree> ");
    loop {
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        std::io::stdout().flush().unwrap();
        print!("\nTree:\n{:#?}\n\nTree> ", rep(super::token::new(&input)));
        std::io::stdout().flush().unwrap();
    }
}
