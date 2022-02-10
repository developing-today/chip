use num_traits::Num;
use std::fmt;

struct NumberData(String);
impl Into<Token> for NumberData {
    fn into(self) -> Token {
        Token::NumberData(self.0)
    }
}
struct Number<T>(T)
where
    T: Num;
impl<T: Num> Into<NumberData> for Number<T>
where
    T: Num + fmt::Display,
{
    fn into(self) -> NumberData {
        NumberData(self.0.to_string())
    }
}
impl<'a, T: Num> From<(char, &mut std::str::Chars<'a>)> for Number<T>
where
    T: Num + fmt::Display + std::str::FromStr,
{
    fn from(val: (char, &mut std::str::Chars<'a>)) -> Self {
        let (c, chars) = val;
        let mut num = String::new();
        num.push(c);
        let mut decimal_point = false;
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                num.push(c);
            } else {
                if c == '.' && !decimal_point {
                    num.push(c);
                    decimal_point = true;
                } else {
                    break;
                }
            }
        }
        Number(num.parse::<T>().unwrap_or(T::zero()))
    }
}

struct Identifier(String);
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        Token::Identifier(self.0)
    }
}
impl<'a> From<(char, &mut std::str::Chars<'a>)> for Identifier {
    fn from(val: (char, &mut std::str::Chars<'a>)) -> Self {
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

struct Data(String);
impl Into<Token> for Data {
    fn into(self) -> Token {
        Token::Data(self.0)
    }
}
impl<'a> From<&mut std::str::Chars<'a>> for Data {
    fn from(val: &mut std::str::Chars<'a>) -> Self {
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
struct Whitespace;
struct Plus;
struct Minus;
struct Multiply;
struct Divide;
struct Modulo;
struct LParen;
struct RParen;
struct Newline;
struct Unknown;

#[derive(Debug, PartialEq)]
pub enum Token {
    NumberData(String),
    Identifier(String),
    Data(String),
    Whitespace,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LParen,
    RParen,
    Newline,
    Unknown,
}
pub(crate) fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars();
    let mut char = chars.next();

    while char.is_some() {
        tokens.push(match char.unwrap() {
            ' ' => Token::Whitespace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '%' => Token::Modulo,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '\n' => Token::Newline,
            c @ '0'..='9' => NumberData::from(Number::<f64>::from((c, &mut chars)).into()).into(),
            c @ 'a'..='z' | c @ 'A'..='Z' => Identifier::from((c, &mut chars)).into(),
            '"' => Data::from(&mut chars).into(),
            _ => Token::Unknown,
        });
        char = chars.next();
    }
    tokens
        .into_iter()
        .filter(|t| t != &Token::Unknown && t != &Token::Whitespace && t != &Token::Newline)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "1 + 1.1";
        let expected = vec![
            Token::NumberData(1.to_string()),
            Token::Plus,
            Token::NumberData(1.1.to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }
}

pub(crate) fn new() {
    let x = "x = 32 + 5 * 2 - 10 + 10 wdfwe 23r 2f.ffasdf ;4 3q'dfw/df s fsdfs.sd;fsf

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
