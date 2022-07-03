use std::fs;

mod ast;
mod parser;
mod run;

fn main() {
    let test = fs::read_to_string("test/multi-machine.specky").unwrap();

    run::run(parser::parse_script(test))
}
