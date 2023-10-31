use crate::{ast::{self, Statement}, token::Token};

use super::{Parser, ParseResult, ParsingError};

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> ParseResult<ast::Statement> {
        let token = self.next()?;

        macro_rules! match_operation {
            (
                $(
                    $token_name:ident =>
                        $($operation:ident ($($details:ident)?))?,
                )*
                $(
                    #
                    $(
                        $arbitrary:ident => $code:tt,
                    )*
                )?
            ) => {
                match token {
                    $(
                        Token::$token_name => $(
                            match_operation!(# $operation $($details)?)
                        )?,
                    )*
                    $($(
                        Token::$arbitrary => $code,
                    )*)?
                    _ => Err(ParsingError::UnexpectedOperator),
                }
            };
            (# $operation:ident Expression) => {
                Ok(Statement::$operation(self.parse_expression()?))
            };
            (# $operation:ident) => {
                Ok(Statement::$operation)
            };
        }

        match_operation!(
            Load => Load(Expression),
            Define => Define(Expression),
            Jump => Jump(Expression),
            Assign => Assign(Expression),
            Overwrite => Overwrite(Expression),
            Swap => Swap(Expression),
        
            And => And(Expression),
            Or => Or(Expression),
            Xor => Xor(Expression),
            Plus => Plus(Expression),
            Minus => Minus(Expression),
            Times => Times(Expression),
            BackSlash => Divide(Expression),
            Modulo => Modulo(Expression),
            Exponential => Exponential(Expression),
        
            Unequal => Unequal(Expression),
            Equal => Equal(Expression),
            LessThan => LessThan(Expression),
            LessThanOrEqual => LessThanOrEqual(Expression),
            GreaterThan => GreaterThan(Expression),
            GreaterThanOrEqual => GreaterThanOrEqual(Expression),
        
            Truthy => Truthy(),
            Falsy => Falsy(),
            Exists => Exists(),
            Empty => Empty(),

            #

            CurlyBracketOpen => {
                let mut kind = None;
                let mut reverse = false;
                let mut newline = true;

                loop {
                    match self.next()? {
                        Token::Modulo => kind = Some(ast::LogKind::Value),
                        Token::At => kind = Some(ast::LogKind::Pointer),
                        Token::Tilde => reverse = !reverse,
                        Token::BackSlash => newline = !newline,
                        Token::CurlyBracketClose => break,
                        _ => return Err(ParsingError::InvalidCharacter)
                    }
                }

                let kind = match kind {
                    Some(kind) => kind,
                    None => return Err(ParsingError::InvalidCharacter),
                };

                Ok(ast::Statement::Log { kind, reverse, newline })
            },
        )
    }
    pub fn parse_statements(&mut self) -> ParseResult<ast::Statements> {
        let mut program = ast::Statements::default();

        while self.next_is_token() {
            program.push(self.parse_statement()?);
        }

        println!("{:?}", self.next());

        Ok(program)
    }
}

