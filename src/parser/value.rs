use std::time::Instant;

use crate::{ast::{self, Float, Integer, SmallInt}, token::Token};

use super::{Parser, ParseResult, ParsingError, error::CodeArea};

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
                    let token = self.next()?;
                    match token {
                        Token::Plus => {},
                        Token::Minus => negative = !negative,
                        Token::IntegerLiteral => {
                            let integer = self.slice()
                                .parse::<SmallInt>()
                                .map(|i| ast::Value::SmallInt(if negative { -i } else { i }))
                                .unwrap_or(ast::Value::Integer(self.slice().parse::<Integer>().unwrap()));
                            return Ok(integer)
                        },
                        Token::FloatLiteral => {
                            let float = self.slice()
                                .parse::<Float>()
                                .unwrap(); // unreachable
                            return Ok(ast::Value::Float(if negative { -float } else { float }))
                        }
                        _ => return Err(ParsingError::SyntaxError {
                            expected: "integer or float".to_string(),
                            found: token,
                            area: CodeArea::from_span(self.span()),
                        }),
                    }
                }
            },

            Some(Token::StringLiteral) => {
                self.next()?;
                let slice = self.slice();

                let mut text = String::new();
                let mut is_escape = false;
                
                let chars = slice[1..slice.len()-1].chars().collect::<Vec<char>>();

                for (i, char) in chars.iter().enumerate() {
                    if is_escape {
                        text.push_str(match char {
                            'r' => "\r",
                            'n' => "\n",
                            't' => "\t",
                            '0' => "\0",
                            '\\'=> "\\",
                            _ => return Err(ParsingError::CustomError {
                                text: "invalid escape character".to_string(),
                                area: CodeArea::from_span(i..i+1),
                            }),
                        })
                    } else if char == &'\\' {
                        is_escape = true
                    } else {
                        text.push(*char)
                    }
                }
                
                if is_escape {
                    return Err(ParsingError::CustomError {
                        text: "not sure if this is reachable".to_string(),
                        area: CodeArea::from_span(self.span()),
                    })
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

            _ => Err(ParsingError::SyntaxError {
                expected: "value".to_string(),
                found: self.next()?,
                area: CodeArea::from_span(self.span()),
            })
        }
    }
}
