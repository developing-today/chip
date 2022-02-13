use rug::Rational;
use std::{
    fmt::Debug,
    str::{Chars, FromStr},
};
fn variant_eq<T>(lhs: &T, rhs: &T) -> bool {
    std::mem::discriminant(lhs) == std::mem::discriminant(rhs)
}
#[derive(Copy, Debug, PartialEq, Clone)]
pub enum Token<'a> {
    NumberData(NumberData<'a>),
    Identifier(Identifier<'a>),
    Data(Data<'a>),
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
    // eof, dot, Op, any, maybe :<>[],?,String Star Slash Percent Caret Ampersand Pipe Tilde Neq Lt
    // but if its not needed, just do it in standard:library
    // and use identifier in the tokenizer
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tokenize() {
//         let input = "1 + 1.1";
//         let expected = vec![
//             Token::NumberData<'a>(Number(1).into()),
//             Token::Plus(Plus),
//             Token::NumberData<'a>(('1', &mut ".1".chars()).into()), // TODO: should stop awakening ancient slumbers
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

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct NumberData<'a>(&'a str); // TODO: link numberT and str, and make it a trait
pub struct Number<T>(T);

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Identifier<'a>(&'a str);
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Data<'a>(&'a str);
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

impl<'a> From<NumberData<'a>> for Token<'a> {
    fn from(val: NumberData<'a>) -> Self {
        Token::NumberData(NumberData::from(val.0))
    }
}
impl<'a> From<Number<&'a str>> for Token<'a> {
    fn from(val: Number<&'a str>) -> Self {
        Token::NumberData(NumberData::from(val.0))
    }
}
impl<'a> From<&'a str> for NumberData<'a> {
    fn from(val: &'a str) -> Self {
        NumberData::<'a>(val)
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
impl<'a, T> From<NumberData<'a>> for Number<T> {
    fn from(x: NumberData) -> Self {
        x.into()
    }
}
impl<'a, T> From<Number<T>> for NumberData<'a>
where
    T: std::fmt::Display,
{
    fn from(val: Number<T>) -> Self {
        let mut str = String::new();
        str.push_str(&val.0.to_string());
        NumberData(&str.as_str())
    }
}
impl<'a> From<(char, &mut Chars<'a>)> for NumberData<'a> {
    fn from(input: (char, &mut Chars<'a>)) -> Self {
        let xx: Number<Rational> = input.into();
        let mut str: String = String::new();
        str.push_str(&xx.0.to_string());
        NumberData(&str.as_str())
    }
}
impl<'a, T: FromStr> From<(char, &mut Chars<'a>)> for Number<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn from(val: (char, &mut Chars<'a>)) -> Self {
        let (c, chars) = val;
        let mut dec = 0;
        let mut rat = false;
        let mut num = String::new();
        num.push(c);
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                num.push(c);

                if rat {
                    dec += 1;
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

impl<'a> From<Identifier<'a>> for Token<'a> {
    fn from(val: Identifier<'a>) -> Self {
        Token::Identifier(Identifier(val.0))
    }
}
impl<'a> From<(char, &mut Chars<'a>)> for Identifier<'a> {
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
        Identifier(&id)
    }
}

impl<'a> From<Data<'a>> for Token<'a> {
    fn from(val: Data<'a>) -> Self {
        Token::Data(Data(val.0))
    }
}
impl<'a> From<&mut Chars<'a>> for Data<'a> {
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
        Data(&id)
    }
}
