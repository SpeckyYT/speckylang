use logos::{Logos, Source};

use crate::ast;
use crate::tokens::Token;

pub fn parse_script(unparsed: String) -> ast::Program {
    let mut lexer = Token::lexer(&unparsed);

    let mut program = ast::Program::default();

    while let Some(Ok(token)) = lexer.next() {
        use Token::*;

        match token {
            // <operator> <symbol/value>
            Load|Define|Jump|Assign|Overwrite|Swap|And|Or|Plus|Minus|Times|Divide|Modulo|Exponential
            |Unequal|Equal|LessThan|LessThanOrEqual|GreaterThan|GreaterThanOrEqual => { // if statements
                let mut next = lexer.next();
                if next.is_none() {
                    panic!("expected value, found EOF");
                }

                let mut reader = 0;
                while let Some(Ok(Reader)) = next {
                    reader += 1;
                    next = lexer.next()
                }

                let value = match next.as_ref().unwrap().as_ref().unwrap() {
                    Null|True|False|StringLiteral|IntegerLiteral|Symbol => {
                        token_to_value(next.unwrap().unwrap(), lexer.slice())
                    },
                    _ => {
                        panic!("sussy");
                    }
                };

                program.push(
                    ast::Operation::Dual(
                        token_to_operator(token),
                        reader,
                        value,
                    )
                );
            },
            // <operator>
            LogValue|LogCurrentAddress
            |Exists|Truthy|Falsy => { // if statements
                program.push(
                    ast::Operation::Mono(
                        token_to_operator(token),
                    )
                );
            },
            _ => panic!("Unexpected stuff: {:?}", token),
        }
    }

    return program;
}

fn token_to_operator(token: Token) -> ast::Operator {
    use Token::*;

    match token {
        Load => ast::Operator::Load,
        Define => ast::Operator::Define,
        Jump => ast::Operator::Jump,
        Assign => ast::Operator::Assign,
        Overwrite => ast::Operator::Overwrite,
        Swap => ast::Operator::Swap,
        And => ast::Operator::And,
        Or => ast::Operator::Or,
        Xor => ast::Operator::Xor,
        Plus => ast::Operator::Plus,
        Minus => ast::Operator::Minus,
        Times => ast::Operator::Times,
        Divide => ast::Operator::Divide,
        Modulo => ast::Operator::Modulo,
        Exponential => ast::Operator::Exponential,
        Unequal => ast::Operator::Unequal,
        Equal => ast::Operator::Equal,
        LessThan => ast::Operator::LessThan,
        LessThanOrEqual => ast::Operator::LessThanOrEqual,
        GreaterThan => ast::Operator::GreaterThan,
        GreaterThanOrEqual => ast::Operator::GreaterThanOrEqual,
        Truthy => ast::Operator::Truthy,
        Falsy => ast::Operator::Falsy,
        Exists => ast::Operator::Exists,
        Empty => ast::Operator::Empty,
        LogValue => ast::Operator::LogValue,
        LogCurrentAddress => ast::Operator::LogCurrentAddress,
        _ => panic!("is not operator"),
    }
}

fn token_to_value(token: Token, content: &str) -> ast::Value {
    use Token::*;

    match token {
        Symbol => ast::Value::Symbol(content.to_string()),
        Null => ast::Value::Null,
        True => ast::Value::Boolean(true),
        False => ast::Value::Boolean(false),
        StringLiteral => ast::Value::String(content.slice(1..(content.len()-1)).unwrap().to_string()),
        IntegerLiteral => ast::Value::Integer(content.parse::<ast::Integer>().unwrap()),
        _ => panic!("is not value"),
    }
}
