// expression -> atom | "(" list ")"
// list -> expression*
// atom -> NUMBERS | STRINGS | SYMBOLS
// SYMBOLS -> ("*", "/", "+", "-", "==", "/=", "t" | "nil")

use crate::lexer::{Token, ParsingError, Result};


pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}


#[derive(Debug)]
pub enum SExpression {
    Number(f64),
    Str(String),
    Symbol(String),
    List(Vec<SExpression>),
}


impl Parser {
    pub fn init(toks: Vec<Token>) -> Self {
        Parser {
            tokens: toks,
            cursor: 0,
        }
    }

    pub fn parse(self: &mut Self) -> Result<SExpression> {
        self.parse_expression()
    }

    fn parse_expression(self: &mut Self) -> Result<SExpression> {
        let res = match self.tokens[self.cursor] {
            Token::OpenParen => self.parse_list(),
            Token::CloseParen => Err(
                ParsingError(String::from("closing parent without opening it"))
            ),
            _ => self.parse_atom(),
        };
        self.cursor += 1;
        res
    }

    fn parse_list(self: &mut Self) -> Result<SExpression> {
        self.cursor += 1; // consume open paren.
        let mut res: Vec<SExpression> = vec![];
        loop {
            if self.tokens[self.cursor] == Token::CloseParen || self.tokens[self.cursor] == Token::End {
                return Ok(SExpression::List(res))
            }
            let exp = self.parse_expression()?;
            res.push(exp);
        }
    }

    fn parse_atom(self: &mut Self) -> Result<SExpression> {
        match &self.tokens[self.cursor] {
            Token::String(s) => Ok(SExpression::Str(s.clone())),
            Token::Symbol(s) => Ok(SExpression::Symbol(s.clone())),
            Token::Number(n) => Ok(SExpression::Number(*n)),
            _ => Err(ParsingError(format!("{:?}", &self.tokens[self.cursor]))),
        }
    }
}
