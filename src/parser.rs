use logos::{Logos, Source};

use crate::ast;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // <operator> <value>
    #[token("|<")]
    Load,
    #[token("[<]")]
    Define,
    #[token("[>]")]
    Jump,
    #[token("<=")]
    Assign,
    #[token("=>")]
    Overwrite,
    #[token("<=>")]
    Swap,

    // <operator> <value>
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token(">-<")]
    Xor,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("^")]
    Exponential,
    #[token("><")]
    Unequal,
    #[token("=")]
    Equal,
    #[token("<")]
    LessThan,
    #[token("=<")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,

    // if statements
    // <operator>
    #[token("?")]
    Truthy,
    #[token("!")]
    Falsy,
    #[token("??")]
    Exists,
    #[token("!!")]
    Empty,

    // <operator>
    #[token("{%}")]
    LogValue,
    #[token("{@}")]
    LogCurrentAddress,

    // symbol
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Symbol,

    // value
    #[token("null")]
    Null,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"/(?:\.|[^\\/])*/")]
    StringLiteral,
    #[regex(r"-?[0-9]+(\.[0-9]+)?")]
    NumberLiteral,

    #[regex(r"\n")]
    Newline,

    #[error]
    #[regex(r"[ \t\n]*|(#[^\n]*\n)", logos::skip)]
    Error,
}

pub fn parse_script(
    mut unparsed: String,
) -> ast::Program {
    unparsed = unparsed.replace("\r\n", "\n");

    let mut lexer = Token::lexer(&unparsed);

    let mut program = ast::Program::new();

    while let Some(token) = lexer.next() {
        use Token::*;

        match token {
            // <operator> <symbol/value>
            Load|Define|Jump|Assign|Overwrite|Swap|And|Or|Plus|Minus|Times|Divide|Modulo|Exponential
            |Unequal|Equal|LessThan|LessThanOrEqual|GreaterThan|GreaterThanOrEqual => { // if statements
                let next = lexer.next();
                if next.is_none() {
                    panic!("expected value, found EOF");
                }
                let value = match next.as_ref().unwrap() {
                    Null|True|False|StringLiteral|NumberLiteral|Symbol => {
                        token_to_value(next.unwrap(), lexer.slice())
                    },
                    Newline => {
                        panic!("sussy");
                    },
                    _ => {
                        panic!("sussy");
                    }
                };

                program.push(
                    ast::Operation::Dual(
                        token_to_operator(token),
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
            Newline => (),
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
        NumberLiteral => ast::Value::Number(content.parse::<ast::Number>().unwrap()),
        _ => panic!("is not value"),
    }
}
