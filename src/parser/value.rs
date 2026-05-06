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
                let span = self.span();

                let mut text = String::new();
                let mut is_escape = false;
                
                let content = &slice[1..slice.len()-1];
                let mut byte_offset = 0;

                for char in content.chars() {
                    let char_start = span.start + 1 + byte_offset;
                    let char_len = char.len_utf8();

                    if is_escape {
                        text.push_str(match char {
                            'r' => "\r",
                            'n' => "\n",
                            't' => "\t",
                            '0' => "\0",
                            '\\' => "\\",
                            c => return Err(ParsingError::InvalidEscapeCharacter {
                                character: c,
                                area: CodeArea::from_span(char_start - 1 .. char_start + char_len),
                            }),
                        });
                        is_escape = false;
                    } else if char == '\\' {
                        is_escape = true;
                    } else {
                        text.push(char);
                    }

                    byte_offset += char_len;
                }
                
                if is_escape {
                    return Err(ParsingError::CustomError {
                        text: "Trailing backslash in string literal".to_string(),
                        area: CodeArea::from_span(span.start + slice.len() - 2 .. span.start + slice.len() - 1),
                    });
                }

                Ok(ast::Value::Text(text))
            }

            Some(Token::Mu) => {
                self.next()?;
                Ok(ast::Value::Time(None))
            }

            Some(Token::Null) => {
                self.next()?;
                Ok(ast::Value::Null)
            }

            _ => {
                let found = self.next()?;
                Err(ParsingError::SyntaxError {
                    expected: "value".to_string(),
                    found,
                    area: CodeArea::from_span(self.span()),
                })
            }
        }
    }
}
