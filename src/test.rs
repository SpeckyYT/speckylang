macro_rules! read {
    ($filename:expr) => {
        std::fs::read_to_string($filename).unwrap()
    };
}

macro_rules! run {
    ($string:expr) => {
        {
            let temp = &$string;
            let mut parser = crate::parser::Parser::new(temp);
            let parsed = parser.parse_statements().unwrap();
            crate::run::run(&parsed)
        }
    };
}

// macro_rules!  {
//     () => {
        
//     };
// }

#[test]
fn bottles_of_beer() {
    let output = run!(read!("test/99-bottles-of-beer.specky"));

    let expected = (1..100).rev()
        .map(|i| [
            i.to_string(),
            "/bottles of beer on the wall,/".to_string(),
            i.to_string(),
            "/bottles of beer./".to_string(),
            "/Take one down, pass it around,/".to_string(),
            (i-1).to_string(),
            "/bottles of beer on the wall,/".to_string(),
            "//".to_string(),
            "".to_string(),
        ].join("\n"))
        .collect::<String>();

    println!("{}", expected);

    assert!(
        output.stdout.contains(&expected)
    )
}

#[test]
fn fibonacci() {
    let output = run!(read!("test/factorial.specky"));

    const VALUE: i32 = 10;

    assert_eq!(output.stdout, (1..=VALUE).fold(1, |acc, x| acc * x).to_string() + "\n")
}

#[test]
fn multi_machine() {
    let output = run!(read!("test/multi-machine.specky"));

    const A: i32 = 50;
    const B: i32 = 10;

    assert_eq!(output.stdout, (A * B).to_string() + "\n");
}
