use crate::{ast::{self, Statement}, token::Token};

use super::{Parser, ParseResult, error::ParsingError, error::CodeArea};

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> ParseResult<Statement> {
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
                    _ => Err(ParsingError::SyntaxError {
                        expected: "operator".to_string(),
                        found: token,
                        area: CodeArea::from_span(self.span()),
                    }),
                }
            };
            (# $operation:ident Expression) => {
                Ok(Statement::$operation(self.parse_expression()?))
            };
            (# $operation:ident Sequential) => {
                {
                    let mut quantity = 1;
                    while let Some(Token::$operation) = self.peek() {
                        self.next()?;
                        quantity += 1;
                    }
                    Ok(Statement::$operation(quantity))
                }
            };
            (# $operation:ident) => {
                Ok(Statement::$operation)
            };
        }

        match_operation!(
            Load => Load(Expression),
            Assign => Assign(Expression),
            Overwrite => Overwrite(Expression),
            Swap => Swap(Expression),
            Tilde => Index(Expression),

            And => And(Expression),
            Or => Or(Expression),
            Xor => Xor(Expression),
            Plus => Plus(Expression),
            Minus => Minus(Expression),
            Asterisk => Times(Expression),
            Backslash => Divide(Expression),
            Percent => Modulo(Expression),
            Circumflex => Exponential(Expression),
        
            Unequal => Unequal(Expression),
            Equal => Equal(Expression),
            LessThan => LessThan(Expression),
            LessThanOrEqual => LessThanOrEqual(Expression),
            GreaterThan => GreaterThan(Expression),
            GreaterThanOrEqual => GreaterThanOrEqual(Expression),
        
            Truthy => Truthy(Sequential),
            Falsy => Falsy(Sequential),
            Exists => Exists(Sequential),
            Empty => Empty(Sequential),

            #

            SquareBracketOpen => {
                // Define => Define(Expression),
                // Jump => Jump(Expression),

                let mut kind = None;

                enum JumpKind {
                    Define,
                    Jump,
                }

                loop {
                    let token = self.next()?;
                    match token {
                        Token::LessThan => kind = Some(JumpKind::Define),   // <
                        Token::GreaterThan => kind = Some(JumpKind::Jump),  // >
                        Token::SquareBracketClose => break,
                        _ => return Err(ParsingError::SyntaxError {
                            expected: "jump option".to_string(),
                            found: token,
                            area: CodeArea::from_span(self.span()),
                        })
                    }
                }

                let expression = self.parse_expression()?;

                match kind {
                    Some(JumpKind::Define) => Ok(Statement::Define(expression)),
                    Some(JumpKind::Jump) => Ok(Statement::Jump(expression)),
                    None => Err(ParsingError::SyntaxError {
                        expected: "`>` or `<` inside of the []".to_string(),
                        found: Token::Mu,
                        area: CodeArea::from_span(self.span()),
                    })
                }
            },

            CurlyBracketOpen => {
                let mut kind = None;
                let mut reader = 0;
                let mut special = false;
                let mut reverse = false;
                let mut newline = true;
                let mut space = 0;
                let mut vertical = false;
                let mut assign = false;

                loop {
                    let token = self.next()?;
                    match token {
                        Token::Percent => kind = Some(ast::LogKind::Value),
                        Token::At => kind = Some(ast::LogKind::Pointer),
                        Token::Reader => reader += 1,
                        Token::Exists => special = !special,
                        Token::Tilde => reverse = !reverse,
                        Token::Backslash => newline = !newline,
                        Token::Empty => space += 1,
                        Token::Circumflex => vertical = !vertical,
                        Token::LessThan => assign = !assign,
                        Token::CurlyBracketClose => break,
                        _ => return Err(ParsingError::SyntaxError {
                            expected: "print option".to_string(),
                            found: token,
                            area: CodeArea::from_span(self.span()),
                        })
                    }
                }

                Ok(Statement::Log {
                    kind,
                    reader,
                    special,
                    reverse,
                    newline,
                    space,
                    vertical,
                    assign,
                })
            },
        )
    }
    pub fn parse_statements(&mut self) -> ParseResult<ast::Statements> {
        let mut program = ast::Statements::default();

        while self.next_is_token() {
            program.push(self.parse_statement()?);
        }

        Ok(program)
    }
}

