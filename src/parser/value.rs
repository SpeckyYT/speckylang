use std::time::Instant;

use num_bigint::BigInt;

use crate::{ast, token::Token};

use super::{Parser, ParseResult, ParsingError};

impl<'a> Parser<'a> {
    pub fn parse_value(&mut self) -> ParseResult<ast::Value> {
        match self.peek() {
            Some(Token::Symbol) => {
                self.next()?;
                Ok(ast::Value::Symbol(self.slice().to_string()))
            }

            Some(Token::True|Token::False) => {
                Ok(ast::Value::Boolean(match self.next()? {
                    Token::True  => true,
                    Token::False => false,
                    _ => unreachable!(),
                }))
            }

            Some(Token::Minus|Token::Plus|Token::IntegerLiteral) =>
                Ok(ast::Value::Integer(self.parse_integer()?)),

            Some(Token::StringLiteral) => {
                self.next()?;
                let slice = self.slice();
                Ok(ast::Value::String(
                    slice[1..slice.len()-1].to_string()
                    .replace("\\n", "\n")
                ))
            }

            Some(Token::Time) => {
                self.next()?;
                Ok(ast::Value::Time(Instant::now()))
            }

            Some(Token::Null) => {
                self.next()?;
                Ok(ast::Value::Null)
            }

            _ => Err(ParsingError::UnexpectedValue)
        }
    }

    fn parse_integer(&mut self) -> ParseResult<ast::Integer> {
        match self.next()? {
            Token::Minus => Ok(-self.parse_integer()?),
            Token::Plus => self.parse_integer(),
            Token::IntegerLiteral => {
                Ok(self.slice().parse::<BigInt>().map_err(|_| ParsingError::InvalidCharacter)?)
            }
            _ => Err(ParsingError::UnexpectedValue),
        }
    }
}
