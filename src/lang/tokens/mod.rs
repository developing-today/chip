use rug::Rational;
use std::{
    fmt::Debug,
    str::{Chars, FromStr},
};
fn variant_eq<T>(lhs: &T, rhs: &T) -> bool {
    std::mem::discriminant(lhs) == std::mem::discriminant(rhs)
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    NumberData(NumberData),
    Identifier(Identifier),
    Data(Data),
    Whitespace(Whitespace),
    Plus(Plus),
    Minus(Minus),
    Multiply(Multiply),
    Divide(Divide),
    Modulo(Modulo),
    LParen(LParen),
    RParen(RParen),
    Newline(Newline),
    Unknown(Unknown),
}

// eof, dot, Op, any, maybe :<>[],?,String Star Slash Percent Caret Ampersand Pipe Tilde Neq Lt
// but if its not needed, just do it in standard:library
// and use identifier in the tokenizer}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tokenize() {
//         let input = "1 + 1.1";
//         let expected = vec![
//             Token::NumberData(Number(1)),
//             Token::Plus(Plus),
//             Token::NumberData(('1', &mut ".1".chars())), // TODO: should stop awakening ancient slumbers
//         ];
//         assert_eq!(tokenize(input), expected);
//     }
// }

#[test]
pub(crate) fn new() {
    let x = r#"x = 32 + 5 * 2 - 10 + 10


    !@#!$%#^$#&^$(& )
    "hello ""clarice"""
    yololololo
    seven eight nine 10 1234123412.1444;
    // 32 + 10 - 10 + 10
    // 42 - 10 + 10
    // 32 + 10
    // 42"#;
    let y = tokenize(x);
    println!("{y:#?}");

    let z = tokenize(
        "x = 10
    y = 20
    z = 30

    result = x + - ( y + ( z - x * y + z ) - x * y )

    if result then
        print result
    else
        print \"\"\"fail\"\"\"
    end
    ",
    );
    println!("{z:#?}");
}
pub enum AtomicToken {
    Whitespace,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LParen,
    RParen,
    Newline,
    DQuote,
    Dot,
    Unknown,
}
impl From<char> for AtomicToken {
    fn from(c: char) -> Self {
        match c {
            ' ' => AtomicToken::Whitespace,
            '+' => AtomicToken::Plus,
            '-' => AtomicToken::Minus,
            '*' => AtomicToken::Multiply,
            '/' => AtomicToken::Divide,
            '%' => AtomicToken::Modulo,
            '(' => AtomicToken::LParen,
            ')' => AtomicToken::RParen,
            '\n' | '\r' => AtomicToken::Newline,
            '"' => AtomicToken::DQuote,
            '.' => AtomicToken::Dot,
            _ => AtomicToken::Unknown,
        }
    }
}
impl From<AtomicToken> for char {
    fn from(token: AtomicToken) -> Self {
        match token {
            AtomicToken::Plus => '+',
            AtomicToken::Minus => '-',
            AtomicToken::Multiply => '*',
            AtomicToken::Divide => '/',
            AtomicToken::Modulo => '%',
            AtomicToken::LParen => '(',
            AtomicToken::RParen => ')',
            AtomicToken::Newline => '\n',
            AtomicToken::DQuote => '"',
            AtomicToken::Dot => '.',
            AtomicToken::Whitespace | AtomicToken::Unknown => ' ',
        }
    }
}
enum DataToken {
    Number,
    Data,
    Identifier,
}

pub(crate) fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = str.chars();
    let mut char = chars.next();
    let mut is_next_char = false;
    let mut is_complex = false;
    while char.is_some() {
        let mut char_unwrapped = char.unwrap();
        tokens.push(match AtomicToken::from(char_unwrapped) {
            AtomicToken::Plus => Token::Plus(Plus),
            AtomicToken::Minus => Token::Minus(Minus),
            AtomicToken::Multiply => Token::Multiply(Multiply),
            AtomicToken::Divide => Token::Divide(Divide),
            AtomicToken::Modulo => Token::Modulo(Modulo),
            AtomicToken::LParen => Token::LParen(LParen),
            AtomicToken::RParen => Token::RParen(RParen),
            AtomicToken::Newline => Token::Newline(Newline),
            AtomicToken::Whitespace => Token::Whitespace(Whitespace()),
            atomic_token => {
                is_complex = true;
                let mut string = String::new();
                let mut data_type = if char_unwrapped.is_numeric() {
                    DataToken::Number
                } else if variant_eq(&atomic_token, &AtomicToken::DQuote) {
                    char = chars.next(); // skip leading "
                    if char.is_some() {
                        char_unwrapped = char.unwrap();
                    } else {
                        break;
                    }
                    DataToken::Data
                } else {
                    DataToken::Identifier
                };

                let mut result = Token::Unknown(Unknown);
                print!("bef|\n");

                while !is_next_char {
                    print!("res{result:?}|str{string:?}|chr{char:?}\n");
                    result = match data_type {
                        DataToken::Number => {
                            print!("number|\n");
                            let mut decimal_point = 0;
                            let mut right_of_decimal_separator = false;
                            string.push(char_unwrapped);
                            char = chars.next();
                            while char.is_some() {
                                char_unwrapped = char.unwrap();
                                match char_unwrapped {
                                    c if c.is_numeric() => {
                                        string.push(c);
                                        if right_of_decimal_separator {
                                            decimal_point += 1;
                                        }
                                    }
                                    c if variant_eq(&c.into(), &AtomicToken::Dot)
                                        && !right_of_decimal_separator =>
                                    {
                                        right_of_decimal_separator = true;
                                    }
                                    c => {
                                        if variant_eq(&c.into(), &AtomicToken::Unknown) {
                                            data_type = DataToken::Identifier;
                                        }
                                        break;
                                    }
                                }
                                char = chars.next();
                            }
                            if variant_eq(&data_type, &DataToken::Number) {
                                is_next_char = true;
                                Token::NumberData(NumberData(
                                    Rational::from_str(
                                        &format!(
                                            "{numerator}/1{denominator_zeroes}",
                                            numerator = string,
                                            denominator_zeroes = &"0".repeat(decimal_point)[..]
                                        )[..],
                                    )
                                    .unwrap()
                                    .to_string(),
                                ))
                            } else {
                                continue;
                            }
                        }

                        DataToken::Identifier => {
                            string.push(char_unwrapped);
                            char = chars.next();
                            while char.is_some() {
                                char_unwrapped = char.unwrap();
                                if !variant_eq(&char_unwrapped.into(), &AtomicToken::Unknown) {
                                    break;
                                }
                                string.push(char_unwrapped);
                                char = chars.next();
                            }
                            is_next_char = true;
                            Token::Identifier(Identifier(string.clone()))
                        }

                        DataToken::Data => {
                            let mut escaped = false;
                            while char.is_some() {
                                char_unwrapped = char.unwrap();
                                if escaped {
                                    if !variant_eq(&char_unwrapped.into(), &AtomicToken::DQuote) {
                                        break;
                                    }
                                } else if variant_eq(&char_unwrapped.into(), &AtomicToken::DQuote) {
                                    escaped = !escaped;
                                    char = chars.next();
                                    continue;
                                }
                                string.push(char_unwrapped);
                                char = chars.next();
                            }
                            // "hello ""clarice"""
                            // hello ""clarice""".is_some'1
                            // hello "clarice"""
                            // hello "clarice""
                            // hello "clarice"
                            is_next_char = true;
                            Token::Data(Data(string.clone()))
                        }
                    };
                }
                result
            }
        });
        if !is_complex & !is_next_char {
            char = chars.next();
        }
        is_next_char = false;
        is_complex = false;
    }

    tokens.retain(|t| -> bool {
        !variant_eq(t, &Token::Whitespace(Whitespace()))
            && !variant_eq(t, &Token::Newline(Newline))
            && !variant_eq(t, &Token::Unknown(Unknown))
    });
    while tokens.last().is_some() && variant_eq(tokens.last().unwrap(), &Token::RParen(RParen)) {
        tokens.pop();
    }
    tokens
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberData(String);
pub struct Number<T>(T);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Data(String);
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Whitespace();
impl From<&mut Chars<'_>> for Whitespace {
    fn from(chars: &mut Chars) -> Self {
        while let Some(char) = chars.next() {
            if !variant_eq(&char.into(), &AtomicToken::Whitespace) {
                break;
            }
        }
        Whitespace()
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Plus;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Minus;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Multiply;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Divide;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Modulo;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct LParen;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct RParen;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Newline;
impl From<&mut Chars<'_>> for Newline {
    fn from(chars: &mut Chars) -> Self {
        while let Some(char) = chars.next() {
            if !variant_eq(&char.into(), &AtomicToken::Newline) {
                break;
            }
        }
        Newline
    }
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Unknown;

impl From<NumberData> for Token {
    fn from(val: NumberData) -> Self {
        Token::NumberData(NumberData::from(val.0))
    }
}
impl From<Number<String>> for Token {
    fn from(val: Number<String>) -> Self {
        Token::NumberData(NumberData::from(val.0))
    }
}
impl From<String> for NumberData {
    fn from(val: String) -> Self {
        NumberData(val)
    }
}

// impl<T> Zero for Number<T>
// where
//     T: Add,
// {
//     fn set_zero(&mut self) {
//         *self = Zero::zero();
//     }

//     fn is_zero(&self) -> bool {
//         true
//     }
// }

// impl<T> Add for Number<T>
// where
//     T: Add,
//     T: Default,
// {
//     fn add(&mut self, x: Number<T>) -> Number<T> {
//         Number(self.0 + x.0)
//     }
// }
impl<T> From<Number<T>> for rug::Rational
where
    T: std::fmt::Display,
{
    fn from(val: Number<T>) -> Self {
        <rug::Rational as FromStr>::from_str(&val.0.to_string()).unwrap()
    }
}

impl<T> From<NumberData> for Number<T> {
    fn from(x: NumberData) -> Self {
        x.into()
    }
}

impl<T> From<Number<T>> for NumberData
where
    T: std::fmt::Display,
{
    fn from(val: Number<T>) -> Self {
        let mut str = String::new();
        str.push_str(&val.0.to_string());
        NumberData(str)
    }
}

// impl From<String> for NumberData {
//     fn from(input: String) -> Self {
//         let xx: Number<Rational> = input;
//         let mut str: String = String::new();
//         str.push(xx.0.to_string().chars());
//         NumberData(&xx.0.to_string())
//     }
// }
// impl<T> From<String> for NumberData {
//     fn from(input: String) -> Self {
//         let xx: Number<T> = input;
//         NumberData(xx.0.to_string())
//     }
// }

// impl<T> From<String> for Number<T>
// where
//     T: FromStr,
//     <T as FromStr>::Err: std::fmt::Debug,
// {
//     fn from(mut val: String) -> Self {
//     }
// }

impl From<Identifier> for Token {
    fn from(val: Identifier) -> Self {
        Token::Identifier(val)
    }
}
impl From<String> for Identifier {
    fn from(val: String) -> Self {
        Identifier(val)
    }
}

impl From<Data> for Token {
    fn from(val: Data) -> Self {
        Token::Data(val)
    }
}

impl From<String> for Data {
    fn from(val: String) -> Self {
        Data(val)
    }
}
