use std::io;
use std::io::Write;

use crate::lang::tokens::Token;

pub fn new(input: &str) -> Vec<Token> {
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
    super::super::tokens::tokenize(input)
}

fn rep(input: &str) -> Vec<Token> {
    new(input)
}

pub fn repl() -> ! {
    std::io::stdout().flush().unwrap();
    print!("Token> ");
    loop {
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        std::io::stdout().flush().unwrap();
        print!("\nToken:\n{:#?}\n\nToken> ", rep(&input));
        std::io::stdout().flush().unwrap();
    }
}
