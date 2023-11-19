use crate::test_read;
use crate::test_run;

const MAX_BOTTLES: usize = 99;

#[test]
fn bottles_of_beer() {
    let output = test_run!(test_read!("test/99-bottles-of-beer.specky"));

    const fn plural(i: usize) -> &'static str { if i == 1 { "" } else { "s" } }

    const MAX_BOTTLES_PLURAL: &str = plural(MAX_BOTTLES);

    let expected = (1..=MAX_BOTTLES).rev()
        .map(|i| {
            let s = plural(i);
            let d = i - 1;
            let sd = plural(d);
            [
                format!("{i} bottle{s} of beer on the wall,"),
                format!("{i} bottle{s} of beer."),
                "Take one down, pass it around,".to_string(),
                format!("{d} bottle{sd} of beer on the wall,"),
                "".to_string(),
                "".to_string(),
            ].join("\n")
        })
        .collect::<String>() + &[
            "No bottles of beer on the wall,",
            "No bottles of beer.",
            "Go to the store, buy some more,",
            &format!("{MAX_BOTTLES} bottle{MAX_BOTTLES_PLURAL} of beer on the wall."),
            "",
        ].join("\n");

    assert_eq!(output.stdout, expected)
}

#[test]
fn fibonacci() {
    let output = test_run!(test_read!("test/factorial.specky"));

    const VALUE: i32 = 10;

    assert_eq!(output.stdout, (1..=VALUE).fold(1, |acc, x| acc * x).to_string() + "\n")
}

#[test]
fn multi_machine() {
    let output = test_run!(test_read!("test/multi-machine.specky"));

    const A: i32 = 50;
    const B: i32 = 10;

    assert_eq!(output.stdout, (A * B).to_string() + "\n");
}

#[test]
fn fizzbuzz() {
    assert_eq!(
        test_run!(test_read!("test/fizzbuzz.specky")).stdout,
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
    let string = test_read!("test/brainfuck.specky");

    let run = |instructions: &str, debug: bool| {
        test_run!(
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
