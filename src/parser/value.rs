use std::time::Instant;

use crate::{ast::{self, Float, Integer}, token::Token};

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

            Some(Token::Minus|Token::Plus|Token::IntegerLiteral|Token::FloatLiteral) => {
                let mut negative = false;
                loop {
                    match self.next()? {
                        Token::Plus => {},
                        Token::Minus => negative = !negative,
                        Token::IntegerLiteral => {
                            let integer = self.slice()
                                .parse::<Integer>()
                                .map_err(|_| ParsingError::InvalidCharacter)?;
                            return Ok(ast::Value::Integer(if negative { -integer } else { integer }))
                        },
                        Token::FloatLiteral => {
                            let float = self.slice()
                                .parse::<Float>()
                                .map_err(|_| ParsingError::InvalidCharacter)?;
                            return Ok(ast::Value::Float(if negative { -float } else { float }))
                        }
                        _ => return Err(ParsingError::UnexpectedValue),
                    }
                }
            },

            Some(Token::StringLiteral) => {
                self.next()?;
                let slice = self.slice();

                let mut text = String::new();
                let mut is_escape = false;
                
                for char in slice[1..slice.len()-1].chars() {
                    if is_escape {
                        text.push_str(match char {
                            'r' => "\r",
                            'n' => "\n",
                            't' => "\t",
                            '0' => "\0",
                            '\\'=> "\\",
                            _ => return Err(ParsingError::InvalidCharacter),
                        })
                    } else {
                        if char == '\\' {
                            is_escape = true
                        } else {
                            text.push(char)
                        }
                    }
                }
                
                if is_escape {
                    return Err(ParsingError::InvalidCharacter)
                }

                Ok(ast::Value::Text(text))
            }

            Some(Token::Mu) => {
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
}
