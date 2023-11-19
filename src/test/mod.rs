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
    ($string:expr) => {
        {
            let temp = &$string;
            let mut parser = crate::parser::Parser::new(temp);
            let parsed = parser.parse_statements().unwrap();
            // println!("{:#?}", parsed);
            crate::run::run(&parsed)
        }
    };
}
