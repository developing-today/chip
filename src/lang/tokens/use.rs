use std::{
    fmt::Debug,
    str::{Chars, FromStr},
};

#[derive(Debug, Clone, PartialEq)]
pub struct NumberData(String);
pub struct Number<T>(T);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Data(String);
#[derive(Copy, Debug, Clone, PartialEq)]
struct Any;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Whitespace(Any);
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
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Unknown;

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

impl From<NumberData> for Number<T> {
    fn from(x: NumberData) -> Self {
        x.into()
    }
}
impl From<Number<T>> for NumberData
where
    T: std::fmt::Display,
{
    fn from(val: Number<T>) -> Self {
        let mut str = String::new();
        str.push_str(&val.0.to_string());
        NumberData(str)
    }
}
impl From<(char, &mut Chars<'_>, String)> for NumberData {
    fn from(input: (char, &mut Chars<'_>, String)) -> Self {
        let xx: Number<T> = input.into();
        NumberData(xx.0.to_string())
    }
}

// struct NumData(T, & String);
// impl<'a, NumData> From<NumData> for Number<T>
// where
//     T: std::fmt::Display,
// {
//     fn from(val: & NumData) -> Self {
//         val.1
//             .push_str(<T as FromStr>::from_str(val.0.to_string()).unwrap())
//     }
// }
impl From<(char, &mut Chars<'_>, String)> for Number<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn from(mut val: (char, &mut Chars<'_>, String)) -> Self {
        let mut dec = 0;
        let mut rat = false;
        val.2.push(val.0);
        while let Some(c) = val.1.next() {
            if val.0.is_digit(10) {
                val.2.push(c);

                if rat {
                    dec += 1;
                }
            } else if !rat && c == '.' {
                rat = true;
            } else {
                break;
            }
        }

        Number(T::from_str(&format!("{0}/1{den}", val.0, den = &"0".repeat(dec)[..])[..]).unwrap())
    }
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

fn main() {
    println!("{}", tokenize("1+2"));
}
