use crate::{ast, token::Token};

use super::{Parser, ParseResult};

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self) -> ParseResult<ast::Expression> {
        let mut reader_count = 0;

        while let Some(Token::Reader) = self.peek() {
            self.next()?;
            reader_count += 1;
        }

        Ok(ast::Expression {
            reader: reader_count,
            value: self.parse_value()?,
        })
    }
}
