use crate::lang::tokens::{RParen, Tokens, Unknown};

use super::super::super::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Leaf {
    pub token: Token,
}
impl From<Token> for Leaf {
    fn from(token: Token) -> Leaf {
        Leaf { token }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Leafs(pub(crate) Vec<Leaf>);
impl From<Tokens> for Leafs {
    fn from(tokens: Tokens) -> Leafs {
        Leafs(tokens.0.into_iter().map(|token| token.into()).collect())
    }
}
impl Leaf {
    pub(crate) fn empty() -> Leaf {
        Token::Unknown(Unknown).into()
    }

    pub(crate) fn prefix_stickiness(self) -> Option<u8> {
        match self.token {
            // Token::NumberData(NumberData) => (13u8),
            // Token::Identifier(Identifier) => (13u8),
            // Token::Data(Data) => (13u8),
            // Token::Whitespace(Whitespace) => (13u8),
            // Token::Plus(Plus) => Some(13u8),
            Token::Minus(Minus) => Some(13u8),
            // Token::Multiply(Multiply) => (13u8),Result<u8, i16>
            // Token::Divide(Divide) => (13u8),
            // Token::Modulo(Modulo) => (13u8),
            // Token::LParen(LParen) => (13u8),
            // Token::RParen(RParen) => (13u8),
            // Token::Newline(Newline) => (13u8),
            // Token::Unknown(Unknown) => (13u8),
            _ => return None,
        }
    }

    pub(crate) fn infix_binding_power(self) -> (u8, u8) {
        match self.token {
            Token::NumberData(NumberData) => (14, 13),
            Token::Identifier(Identifier) => (14, 13),
            Token::Data(Data) => (14, 13),
            Token::Whitespace(Whitespace) => (14, 13),
            Token::Plus(Plus) => (14, 13),
            Token::Minus(Minus) => (14, 13),
            Token::Multiply(Multiply) => (14, 13),
            Token::Divide(Divide) => (14, 13),
            Token::Modulo(Modulo) => (14, 13),
            Token::LParen(LParen) => (14, 13),
            Token::RParen(RParen) => (14, 13),
            Token::Newline(Newline) => (14, 13),
            Token::Unknown(Unknown) => (14, 13),
        }
    }
    pub(crate) fn postfix_binding_power(self) -> Option<u8> {
        match self.token {
            // Token::NumberData(NumberData) => (14u8),
            // Token::Identifier(Identifier) => (14u8),
            // Token::Data(Data) => (14u8),
            // Token::Whitespace(Whitespace) => (14u8),
            // Token::Plus(Plus) => (14u8),
            // Token::Minus(Minus) => (14u8),
            // Token::Multiply(Multiply) => (14u8),
            // Token::Divide(Divide) => (14u8),
            // Token::Modulo(Modulo) => (14u8),
            // Token::LParen(LParen) => (14u8),
            Token::RParen(RParen) => Some(14u8),
            // Token::Newline(Newline) => (14u8),
            // Token::Unknown(Unknown) => (14u8),
            _ => return None,
        }
    }

    pub(crate) fn stickiness(&self) -> ((u8, u8), bool) {
        let infix_binding_power = &self.clone().infix_binding_power();
        if let Some(postfix_binding_power) = &self.clone().postfix_binding_power() {
            if infix_binding_power.1 <= *postfix_binding_power {
                return ((0, *postfix_binding_power), false);
            }
        }
        return (*infix_binding_power, true);
    }
    pub(crate) fn ternary(self) -> bool {
        match self.token {
            _ => false,
        }
    }
    pub(crate) fn grouping(self) -> Option<Token> {
        match self.token {
            Token::LParen(LParen) => Token::RParen(RParen).into(),
            // prefix( stuff expect)(postfix)
            // infix? stuff expect:(infix)
            _ => None,
        }
    }
}
