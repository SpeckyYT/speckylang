use std::fmt::Debug;
use num_bigint::BigInt;

pub type Integer = BigInt;

#[derive(Debug, Clone, Copy)]
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
    Integer(Integer),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Dual(Operator, u8, Value),
    Mono(Operator),
}

pub type Program = Vec<Operation>;
