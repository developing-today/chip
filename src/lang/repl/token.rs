use std::io;
use std::io::Write;

pub fn new() -> ! {
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
    print!("Token> ");
    loop {
        rep();
    }
}
fn rep() {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    print!(
        "Token: {:#?}\nToken> ",
        super::super::tokenizer::tokenize(&input)
    );
}
