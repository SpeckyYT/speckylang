use std::{fs, path::PathBuf, time::{Duration, Instant}, process};
use clap::Parser;

mod ast;
mod token;
mod parser;
mod run;

#[cfg(test)]
mod test;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[arg(short, long)]
    benchmark: bool,
}

fn main() {
    let args = Args::parse();

    let test = fs::read_to_string(args.file).unwrap();

    let parsed = parse(&test);

    match args.benchmark {
        false => run(&parsed),
        true => {
            let mut min = Duration::MAX;
            let mut max = Duration::ZERO;
            let mut all = vec![];

            let start = Instant::now();

            for _ in 0..100000 {
                if start.elapsed() > Duration::from_secs_f64(10.0) {
                    break
                }

                let operations = parsed.clone();

                let begin = Instant::now();
                run(&operations);
                let taken = begin.elapsed();

                min = min.min(taken);
                max = max.max(taken);
                all.push(taken);
            }

            println!("times: {}", all.len());
            println!("min: {:?}", min);
            println!("max: {:?}", max);
            println!("average: {:?}", all.iter().sum::<Duration>() / all.len() as u32)
        }
    };
}

fn parse(code: &str) -> Vec<ast::Statement> {
    let mut parser = parser::Parser::new(code);
    match parser.parse_statements() {
        Ok(statements) => statements,
        Err(error) => {
            parser::error::print_error(code, error);
            process::exit(1)
        }
    }
}

fn run(parsed: &Vec<ast::Statement>) {
    run::run(parsed);
}