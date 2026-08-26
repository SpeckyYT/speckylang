//! # SpeckyLang
//!
//! This library provides the core functionality for the SpeckyLang programming language,
//! including parsing, AST representation, and execution.

pub mod ast;
pub mod compiler;
pub mod parser;
pub mod run;
#[cfg(test)]
pub mod test;
pub mod token;

pub use ast::{Expression, Statement, Value};
pub use parser::Parser;
pub use run::{run, RunOutput};

/// C ABI entry point used by generated LLVM modules.
#[no_mangle]
pub extern "C" fn specky_run(source: *const u8, length: u64) -> i32 {
    if source.is_null() {
        return 1;
    }

    let Ok(length) = usize::try_from(length) else {
        eprintln!("SpeckyLang source is too large for this target");
        return 1;
    };
    let bytes = unsafe { std::slice::from_raw_parts(source, length) };
    let Ok(code) = std::str::from_utf8(bytes) else {
        eprintln!("SpeckyLang source is not valid UTF-8");
        return 1;
    };

    let mut parser = Parser::new(code);
    let parsed = match parser.parse_statements() {
        Ok(statements) => statements,
        Err(error) => {
            parser::error::print_error(code, error);
            return 1;
        }
    };

    run(&parsed);
    0
}
