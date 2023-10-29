mod value;
mod expression;
mod statement;

use std::ops::Range;

use logos::Lexer;

use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingError {
    UnexpectedOperator,
    UnexpectedValue,
    UnexpectedEndOfFile,
    InvalidCharacter,
}

pub type ParseResult<T> = Result<T, ParsingError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            lexer: Lexer::new(string),
        }
    }
    
    fn next(&mut self) -> ParseResult<Token> {
        match self.next_raw() {
            Some(result) => {
                match result {
                    Ok(token) => Ok(token),
                    Err(()) => self.next()
                }
            },
            None => Err(ParsingError::UnexpectedEndOfFile),
        }
    }
    fn next_raw(&mut self) -> Option<Result<Token, ()>> {
        self.lexer.next()
    }
    fn next_is_token(&self) -> bool {
        self.clone().next().is_ok()
    }
    #[allow(dead_code)]
    pub fn span(&self) -> Range<usize> {  self.lexer.span() }
    pub fn slice(&self) -> &str { self.lexer.slice() }
    fn peek(&self) -> Option<Token> {
        let mut lexer = self.lexer.clone();
        lexer.next()?.ok()
    }
}
