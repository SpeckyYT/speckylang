macro_rules! run {
    ($filename:expr) => {
        crate::run::run(crate::parser::parse_script(std::fs::read_to_string($filename).unwrap()))
    };
}

#[test]
fn bottles_of_beer() {
    let output = run!("test/99-bottles-of-beer.specky");

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
        output.contains(&expected)
    )
}

#[test]
fn fibonacci() {
    let output = run!("test/factorial.specky");

    const VALUE: i32 = 10;

    assert_eq!(output, (1..=VALUE).fold(1, |acc, x| acc * x).to_string() + "\n")
}

#[test]
fn multi_machine() {
    let output = run!("test/multi-machine.specky");

    const A: i32 = 50;
    const B: i32 = 10;

    assert_eq!(output, (A * B).to_string() + "\n");
}
