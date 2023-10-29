use std::time::Instant;

use num_bigint::BigInt;

pub type Integer = BigInt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Value {
    Symbol(String),
    Boolean(bool),
    Integer(Integer),
    String(String),
    Time(Instant),
    Null,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub reader: usize,
    pub value: Value,
} 

#[derive(Debug, Clone)]
pub enum Statement {
    Load(Expression),
    Define(Expression),  // address definition
    Jump(Expression),    // jump to address
    Assign(Expression),
    Overwrite(Expression),
    Swap(Expression),

    And(Expression),
    Or(Expression),
    Xor(Expression),
    Plus(Expression),
    Minus(Expression),
    Times(Expression),
    Divide(Expression),
    Modulo(Expression),
    Exponential(Expression),

    Unequal(Expression),
    Equal(Expression),
    LessThan(Expression),
    LessThanOrEqual(Expression),
    GreaterThan(Expression),
    GreaterThanOrEqual(Expression),

    Truthy,
    Falsy,
    Exists,
    Empty,

    LogValue,
    LogCurrentAddress,
}

pub type Statements = Vec<Statement>;
