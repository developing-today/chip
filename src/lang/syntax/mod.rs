pub mod tree;

use super::tokenizer::tokenize;

fn main() {
    let mut tokenizer = Tokenizer::new("fn main() { }");
    let tokens = tokenizer.tokenize();
    println!("{:?}", tokens);
}
