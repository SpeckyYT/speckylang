use std::{time::Instant, hash::Hash};

use num_bigint::BigInt;
use num_bigfloat::BigFloat;

pub type Text = String;
pub type Integer = BigInt;
pub type Float = BigFloat;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Value {
    JumpAddress(usize),
    Symbol(String),
    Boolean(bool),
    Integer(Integer),
    Float(Float),
    Text(Text),
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
    Index(Expression),

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

    Truthy(usize),
    Falsy(usize),
    Exists(usize),
    Empty(usize),

    Log {
        kind: Option<LogKind>,
        reader: usize,
        special: bool,
        reverse: bool,
        newline: bool,
        space: usize,
        vertical: bool,
        assign: bool,
    },
}

#[derive(Debug, Clone)]
pub enum LogKind {
    Value,
    Pointer,
}

pub type Statements = Vec<Statement>;
