use itertools::Itertools;

use crate::ast;
use crate::test_read;
use crate::test_run;

const MAX_BOTTLES: usize = 99;

#[test]
fn bottles_of_beer() {
    let output = test_run!(test_read!("examples/99-bottles-of-beer.specky"));

    const fn plural(i: usize) -> &'static str { if i == 1 { "" } else { "s" } }

    const MAX_BOTTLES_PLURAL: &str = plural(MAX_BOTTLES);

    let mut expected = String::with_capacity(11600);

    for i in (1..=MAX_BOTTLES).rev() {
        let s = plural(i);
        let d = i - 1;
        let sd = plural(d);

        expected.push_str(&format!("{i} bottle{s} of beer on the wall,\n"));
        expected.push_str(&format!("{i} bottle{s} of beer.\n"));
        expected.push_str("Take one down, pass it around,\n");
        expected.push_str(&format!("{d} bottle{sd} of beer on the wall,\n\n"));
    }

    expected.push_str("No bottles of beer on the wall,\n");
    expected.push_str("No bottles of beer.\n");
    expected.push_str("Go to the store, buy some more,\n");
    expected.push_str(&format!("{MAX_BOTTLES} bottle{MAX_BOTTLES_PLURAL} of beer on the wall.\n"));

    assert_eq!(output.stdout, expected)
}

#[test]
fn factorial() {
    let output = test_run!(test_read!("examples/factorial.specky"));

    const VALUE: i32 = 10;

    assert_eq!(output.stdout, (1..=VALUE).fold(1, |acc, x| acc * x).to_string() + "\n")
}

#[test]
fn multi_machine() {
    let output = test_run!(test_read!("examples/multi-machine.specky"));

    const A: i32 = 50;
    const B: i32 = 10;

    assert_eq!(output.stdout, (A * B).to_string() + "\n");
}

#[test]
fn fizzbuzz() {
    assert_eq!(
        test_run!(test_read!("examples/fizzbuzz.specky")).stdout,
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
fn bubblesort() {
    const ARRAYS: &[&[u16]] = &[
        &[3,2,1],
        &[64,88,2,45,51,5,6,60,98,67],
        &[8573,6860,5769,1162,1729,4969,5851,5008,6365,4900,5722,1605],
    ];
    for &array in ARRAYS {
        let mut sorted = array.to_vec();
        sorted.sort();

        assert_eq!(
            test_run!(
                test_read!("examples/bubblesort.specky"),
                [
                    ast::Statement::Assign(ast::Expression { value: ast::Value::Text(array.iter().join(" ")), reader: 0 }),
                ]
            ).stdout.lines().rev().find_map(|s| (!s.is_empty()).then_some(s.trim())),
            Some(sorted.iter().join(" ").as_str())
        )
    }
}

#[test]
fn brainfuck() {
    let string = test_read!("examples/brainfuck.specky");

    let run = |instructions: &str, debug: bool| {
        test_run!(
            string,
            [
                ast::Statement::Assign(ast::Expression { value: ast::Value::Text(instructions.to_string()), reader: 0 }),
                ast::Statement::Assign(ast::Expression { value: ast::Value::Boolean(debug), reader: 0 }),
            ]
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

    /*
    // THIS IS SLOW
    const QUINE: &str = ">+++++>+++>+++>+++++>+++>+++>+++++>++++++>+>++>+++>++++>++++>+++>+++>+++++>+>+>++++>+++++++>+>+++++>+>+>+++++>++++++>+++>+++>++>+>+>++++>++++++>++++>++++>+++>+++++>+++>+++>++++>++>+>+>+>+>++>++>++>+>+>++>+>+>++++++>++++++>+>+>++++++>++++++>+>+>+>+++++>++++++>+>+++++>+++>+++>++++>++>+>+>++>+>+>++>++>+>+>++>++>+>+>+>+>++>+>+>+>++++>++>++>+>+++++>++++++>+++>+++>+++>+++>+++>+++>++>+>+>+>+>++>+>+>++++>+++>+++>+++>+++++>+>+++++>++++++>+>+>+>++>+++>+++>+++++++>+++>++++>+>++>+>+++++++>++++++>+>+++++>++++++>+++>+++>++>++>++>++>++>++>+>++>++>++>++>++>++>++>++>++>+>++++>++>++>++>++>++>++>++>+++++>++++++>++++>+++>+++++>++++++>++++>+++>+++>++++>+>+>+>+>+++++>+++>+++++>++++++>+++>+++>+++>++>+>+>+>++++>++++[[>>>+<<<-]<]>>>>[<<[-]<[-]+++++++[>+++++++++>++++++<<-]>-.>+>[<.<<+>>>-]>]<<<[>>+>>>>+<<<<<<-]>++[>>>+>>>>++>>++>>+>>+[<<]>-]>>>-->>-->>+>>+++>>>>+[<<]<[[-[>>+<<-]>>]>.[>>]<<[[<+>-]<<]<<]";
    assert_eq!(
        run(QUINE, false),
        QUINE,
    );
    */
}
