use std::str::{Chars, FromStr};

use fraction::Signed;
use num_traits::abs;
use rug::Rational;
fn variant_eq<T>(lhs: &T, rhs: &T) -> bool {
    std::mem::discriminant(lhs) == std::mem::discriminant(rhs)
}
#[derive(Debug, Clone, PartialEq)]
pub struct NumberData(String);
pub struct Number<T>(T);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Data(String);
#[derive(Debug, Clone, PartialEq)]
struct Any;
#[derive(Debug, Clone, PartialEq)]
pub struct Whitespace(Any);
#[derive(Debug, Clone, PartialEq)]
pub struct Plus;
#[derive(Debug, Clone, PartialEq)]
pub struct Minus;
#[derive(Debug, Clone, PartialEq)]
pub struct Multiply;
#[derive(Debug, Clone, PartialEq)]
pub struct Divide;
#[derive(Debug, Clone, PartialEq)]
pub struct Modulo;
#[derive(Debug, Clone, PartialEq)]
pub struct LParen;
#[derive(Debug, Clone, PartialEq)]
pub struct RParen;
#[derive(Debug, Clone, PartialEq)]
pub struct Newline;
#[derive(Debug, Clone, PartialEq)]
pub struct Unknown;

#[derive(Debug, PartialEq)]
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

pub(crate) fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars();
    let mut char = chars.next();

    while char.is_some() {
        tokens.push(match char.unwrap() {
            ' ' => Token::Whitespace(Whitespace(Any)),
            '+' => Token::Plus(Plus),
            '-' => Token::Minus(Minus),
            '*' => Token::Multiply(Multiply),
            '/' => Token::Divide(Divide),
            '%' => Token::Modulo(Modulo),
            '(' => Token::LParen(LParen),
            ')' => Token::RParen(RParen),
            '\n' => Token::Newline(Newline),
            c @ '0'..='9' => NumberData::from((c, &mut chars)).into(),
            c @ 'a'..='z' | c @ 'A'..='Z' => Identifier::from((c, &mut chars)).into(),
            '"' => Data::from(&mut chars).into(),
            _ => Token::Unknown(Unknown),
        });
        char = chars.next();
    }
    tokens.retain(|t| -> bool {
        !variant_eq(t, &Token::Whitespace(Whitespace(Any)))
            && !variant_eq(t, &Token::Newline(Newline))
            && !variant_eq(t, &Token::Unknown(Unknown))
    });
    while tokens.last().is_some() && variant_eq(tokens.last().unwrap(), &Token::RParen(RParen)) {
        tokens.pop();
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "1 + 1.1";
        let expected = vec![
            Token::NumberData(NumberData(1.to_string())),
            // Token::Plus,
            Token::NumberData(NumberData(1.1.to_string())),
        ];
        assert_eq!(tokenize(input), expected);
    }
}

#[test]
pub(crate) fn new() {
    let x = "x = 32 + 5 * 2 - 10 + 10

    !@#!$%#^$#&^$(& )
    \"hello \"\"clarice\"\"\"
    yololololo
    seven eight nine 10 1234123412.1444";
    // 32 + 10 - 10 + 10
    // 42 - 10 + 10
    // 32 + 10
    // 42
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

impl From<NumberData> for Token {
    fn from(val: NumberData) -> Self {
        Token::NumberData(NumberData(val.0))
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
        NumberData(val.0.to_string())
    }
}
impl From<(char, &mut Chars<'_>)> for NumberData {
    fn from(input: (char, &mut Chars<'_>)) -> Self {
        let xx: Number<Rational> = input.into();
        NumberData(xx.0.to_string())
    }
}
impl<'a, T: FromStr> From<(char, &mut Chars<'a>)> for Number<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn from(val: (char, &mut Chars<'a>)) -> Self {
        let (c, chars) = val;
        let mut num = String::new();
        num.push(c);

        let mut dec = 0;
        let mut rat = false;
        let mut first_dec = true;

        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                num.push(c);

                if rat {
                    if first_dec {
                        dec += 1;
                        first_dec = false;
                    } else {
                        dec += 1;
                    }
                }
            } else if !rat && c == '.' {
                rat = true;
            } else {
                break;
            }
        }

        Number(T::from_str(&format!("{num}/1{den}", den = &"0".repeat(dec)[..])[..]).unwrap())
    }
}

impl From<Identifier> for Token {
    fn from(val: Identifier) -> Self {
        Token::Identifier(Identifier(val.0))
    }
}
impl<'a> From<(char, &mut Chars<'a>)> for Identifier {
    fn from(val: (char, &mut Chars<'a>)) -> Self {
        let (c, chars) = val;
        let mut id = String::new();
        id.push(c);
        while let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                id.push(c);
            } else {
                break;
            }
        }
        Identifier(id)
    }
}

impl From<Data> for Token {
    fn from(val: Data) -> Self {
        Token::Data(Data(val.0))
    }
}
impl<'a> From<&mut Chars<'a>> for Data {
    fn from(val: &mut Chars<'a>) -> Self {
        let mut id = String::new();

        let mut escaped = false;
        let mut peekable = val.peekable();
        while let Some(c) = peekable.next() {
            if c == '"'
                && id.len() > 0
                && !escaped
                && peekable.peek().is_some()
                && peekable.peek() != Some(&'"')
            {
                break;
            }
            if (c == '"') == escaped {
                id.push(c);
                if escaped {
                    escaped = false
                }
            } else {
                if escaped {
                    break;
                }
                escaped = true;
            }
        }
        // "hello ""clarice"""
        // hello ""clarice"""
        // hello "clarice"""
        // hello "clarice""
        // hello "clarice"
        Data(id)
    }
}
