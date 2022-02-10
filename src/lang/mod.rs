mod tokenizer;

#[allow(dead_code)] // this fn is for testing
fn add(x: u8) -> u8 {
    x + 1
}

#[allow(dead_code)] // this fn is for testing
pub(crate) fn main() {
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
    tokenizer::main()
}
