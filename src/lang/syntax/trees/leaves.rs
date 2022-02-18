// use super::super::super::tokens::Token;

// #[derive(Debug)]
// pub struct Leaf {
//     pub token: Token,
// }
// impl Leaf {
//     pub fn new(token: Token) -> Leaf {
//         Leaf { token }
//     }

//     fn prefix_binding_power(self) -> Result<u8, i16> {
//         match self.token {
//             // Token::NumberData(NumberData) => (13u8),
//             // Token::Identifier(Identifier) => (13u8),
//             // Token::Data(Data) => (13u8),
//             // Token::Whitespace(Whitespace) => (13u8),
//             Token::Plus(Plus) => Ok(13u8),
//             Token::Minus(Minus) => Ok(13u8),
//             // Token::Multiply(Multiply) => (13u8),
//             // Token::Divide(Divide) => (13u8),
//             // Token::Modulo(Modulo) => (13u8),
//             // Token::LParen(LParen) => (13u8),
//             // Token::RParen(RParen) => (13u8),
//             // Token::Newline(Newline) => (13u8),
//             // Token::Unknown(Unknown) => (13u8),
//             _ => return Err(-13i16),
//         }
//     }

//     fn infix_binding_power(self) -> (u8, u8) {
//         match self.token {
//             Token::NumberData(NumberData) => (14, 13),
//             Token::Identifier(Identifier) => (14, 13),
//             Token::Data(Data) => (14, 13),
//             Token::Whitespace(Whitespace) => (14, 13),
//             Token::Plus(Plus) => (14, 13),
//             Token::Minus(Minus) => (14, 13),
//             Token::Multiply(Multiply) => (14, 13),
//             Token::Divide(Divide) => (14, 13),
//             Token::Modulo(Modulo) => (14, 13),
//             Token::LParen(LParen) => (14, 13),
//             Token::RParen(RParen) => (14, 13),
//             Token::Newline(Newline) => (14, 13),
//             Token::Unknown(Unknown) => (14, 13),
//         }
//     }
//     fn postfix_binding_power(self) -> Result<u8, i16> {
//         match self.token {
//             // Token::NumberData(NumberData) => (14u8),
//             // Token::Identifier(Identifier) => (14u8),
//             // Token::Data(Data) => (14u8),
//             // Token::Whitespace(Whitespace) => (14u8),
//             // Token::Plus(Plus) => (14u8),
//             // Token::Minus(Minus) => (14u8),
//             // Token::Multiply(Multiply) => (14u8),
//             // Token::Divide(Divide) => (14u8),
//             // Token::Modulo(Modulo) => (14u8),
//             // Token::LParen(LParen) => (14u8),
//             Token::RParen(RParen) => Ok(14u8),
//             // Token::Newline(Newline) => (14u8),
//             // Token::Unknown(Unknown) => (14u8),
//             _ => return Err(-14i16),
//         }
//     }
// }
