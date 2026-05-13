//! # SpeckyLang
//!
//! This library provides the core functionality for the SpeckyLang programming language,
//! including parsing, AST representation, and execution.

pub mod ast;
pub mod parser;
pub mod run;
pub mod token;
#[cfg(test)]
pub mod test;

pub use ast::{Expression, Statement, Value};
pub use parser::Parser;
pub use run::{run, RunOutput};
