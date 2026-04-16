use std::{time::Instant, hash::Hash};

use num_bigint::BigInt;
use num_bigfloat::BigFloat;

pub type Text = String;
pub type Integer = BigInt;
pub type Float = BigFloat;
pub type SmallInt = i128;

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd)]
pub enum Value {
    Symbol(String),
    Boolean(bool),
    Integer(Integer),
    SmallInt(SmallInt),
    Float(Float),
    Text(Text),
    Time(Option<Instant>),
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
    PPercent(Expression),
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
        reverse: bool,
        newline: bool,
        space: usize,
        vertical: bool,
        assign: bool,
    },

    Input,
}

#[derive(Debug, Clone, Copy)]
pub enum LogKind {
    Value(LogValue),
    Type,
    Memory(LogMemory),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogValue {
    pub reader: usize,
    pub pretty: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogMemory {
    pub sort: bool,
}

pub type Statements = Vec<Statement>;
