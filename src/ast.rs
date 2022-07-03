use std::fmt::Debug;

pub type Number = i128;

#[derive(Debug)]
pub enum Operator {
    Load,
    Define,
    Jump,
    Assign,
    Overwrite,
    Swap,
    And,
    Or,
    Xor,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Exponential,
    Unequal,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Truthy,
    Falsy,
    Exists,
    Empty,
    LogValue,
    LogCurrentAddress,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Value {
    Symbol(String),
    Boolean(bool),
    Number(Number),
    String(String),
    Null,
}

#[derive(Debug)]
pub enum Operation {
    Dual(Operator, Value),
    Mono(Operator),
}

pub type Program = Vec<Operation>;
