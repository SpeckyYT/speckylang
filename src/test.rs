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
            println!("{:#?}", parsed);
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

#[test]
fn print_test() {
    let output = run!(r"|< ab {@}");
    assert_eq!(output.stdout, "ab\n");

    let output = run!(r"|< ab {~@}");
    assert_eq!(output.stdout, "ba\n");

    let output = run!(r"|< ab{@\}");
    assert_eq!(output.stdout, "ab");

    let output = run!(r"|< ab {~@\}");
    assert_eq!(output.stdout, "ba");
}

#[test]
fn sequential_test() {
    let bools = ["true", "false"];
    let results = ["holy shit", "kinda sus"];
    for i in 0..bools.len() {
        let output = run!(format!(r"|< a <= {} ??? |< new <= /{}/ {{%}} |< a !!! |< old <= /{}/ {{%}}", bools[i], results[0], results[1]));
        assert_eq!(output.stdout, format!("/{}/\n", results[i]));
    }
}

#[test]
fn fizzbuzz() {
    assert_eq!(
        run!(read!("test/fizzbuzz.specky")).stdout,
        (1..=1000)
        .map(|i|{
            let mut string = String::with_capacity(8);
            if i % 3 == 0 { string += "Fizz" }
            if i % 5 == 0 { string += "Buzz" }
            if string.is_empty() { string += &i.to_string() }
            string + "\n"
        })
        .collect::<String>()
    )
}

#[test]
fn brainfuck() {
    let string = read!("test/brainfuck.specky");

    let run = |instructions: &str, debug: bool| {
        run!(
            string
            .replace("{INPUT}", instructions)
            .replace("{DEBUG}", &debug.to_string())
        ).stdout
    };

    // https://github.com/saulpw/brainfuck/blob/master/tests
    assert_eq!(
        run("+++++[>+++++++>++<<-]>.>.][", false),
        "#\n"
    );

    assert_eq!(
        run(".+[.+]", false),
        (0..=255).into_iter().map(|i| char::from(i)).collect::<String>()
    );
}
