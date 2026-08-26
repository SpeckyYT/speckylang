use std::{fs, path::PathBuf, process, time::{Duration, Instant}};
use clap::Parser;

use speckylang::compiler::emit_llvm;
use speckylang::parser::Parser as SpeckyParser;
use speckylang::run;

mod compile;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[arg(short, long)]
    benchmark: bool,
    /// Emit LLVM IR instead of running the program.
    #[arg(long, conflicts_with_all = ["benchmark", "compile"])]
    emit_llvm: bool,
    /// Build a native executable using LLVM/Clang.
    #[arg(short = 'c', long, conflicts_with_all = ["benchmark", "emit_llvm"])]
    compile: bool,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(&args.file).unwrap();

    if args.emit_llvm {
        let ir = emit_llvm(&code);
        match args.output {
            Some(path) => fs::write(path, ir).unwrap(),
            None => print!("{ir}"),
        }
        return;
    }

    if args.compile {
        compile::compile(&code, args.output.as_deref());
        return;
    }

    let parsed = parse(&code);

    match args.benchmark {
        false => {
            run(&parsed);
        },
        true => {
            let mut min = Duration::MAX;
            let mut max = Duration::ZERO;
            let mut all = vec![];

            let start = Instant::now();

            for _ in 0..100000 {
                if start.elapsed() > Duration::from_secs_f64(20.0) {
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

fn parse(code: &str) -> Vec<speckylang::Statement> {
    let mut parser = SpeckyParser::new(code);
    match parser.parse_statements() {
        Ok(statements) => statements,
        Err(error) => {
            speckylang::parser::error::print_error(code, error);
            process::exit(1)
        }
    }
}
