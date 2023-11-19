use itertools::Itertools;

use crate::test_run;

#[test]
fn print_test() {
    let output = test_run!(r"|< ab {@}");
    assert_eq!(output.stdout, "ab\n");

    let output = test_run!(r"|< ab {~@}");
    assert_eq!(output.stdout, "ba\n");

    let output = test_run!(r"|< ab {@\}");
    assert_eq!(output.stdout, "ab");

    let output = test_run!(r"|< ab {~@\}");
    assert_eq!(output.stdout, "ba");
}

#[test]
fn sequential_test() {
    let bools = ["true", "false"];
    let results = ["holy shit", "kinda sus"];
    for i in 0..bools.len() {
        let output = test_run!(format!(r"|< a <= {} ??? |< new <= /{}/ {{%}} |< a !!! |< old <= /{}/ {{%}}", bools[i], results[0], results[1]));
        assert_eq!(output.stdout, format!("/{}/\n", results[i]));
    }
}

#[test]
fn reader() {
    let mut script = String::new();
    let mut expected_output = String::new();

    const NUMBERS: std::ops::Range<usize> = 1..50;

    for (i, n) in NUMBERS.enumerate() {
        script += &format!("|< {n} <= {i}\n"); // 0 <= 1; 1 <= 2; etc
    }
    for n in NUMBERS.rev() {
        for i in 0..n {
            script += &format!("|< {}{n} {{@}}\n", "§".repeat(i));
            expected_output += &format!("{}\n", n - i);
        }
    }

    assert_eq!(test_run!(script).stdout, expected_output);
}

#[test]
fn circular_reader() {
    for size in 2..=15 {
        let numbers: Vec<_> = (0..size).collect();

        let mut definition = numbers.iter()
            .circular_tuple_windows()
            .map(|(a,b)| format!("|< {a} <= {b}\n"))
            .collect::<String>();

        let mut expected_output = String::new();

        for i in 0..(3 * size) {
            for j in 0..size {
                definition += &format!("|< {}{} {{@}}\n", "§".repeat(i), numbers[j]);
                expected_output += &format!("{}\n", numbers[(i + j) % size]);
            }
        }

        let ran = test_run!(definition);
        assert_eq!(ran.stdout, expected_output);
    }
}
