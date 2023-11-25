mod features;
mod examples;

#[macro_export]
macro_rules! test_read {
    ($filename:expr) => {
        std::fs::read_to_string($filename).unwrap()
    };
}

#[macro_export]
macro_rules! test_run {
    ($string:expr $(, [$($input:expr),*])?) => {
        {
            let temp = &$string;
            let mut parser = crate::parser::Parser::new(temp);
            #[allow(unused_mut)]
            let mut parsed = parser.parse_statements().unwrap();

            $($(
                if let Some(index) = parsed.iter().position(|v| matches!(v, crate::ast::Statement::Input)) {
                    parsed[index] = $input;
                }
            )*)?

            // println!("{:#?}", parsed);
            let mut ran = crate::run::run(&parsed);

            ran.stdout = ran.stdout
                .trim_start_matches("input your brainfuck program: do you want debug mode? ").to_string();

            ran
        }
    };
}
