use std::time::Instant;

use fnv::FnvHashMap;

type SpeckyDataContainer<V> = FnvHashMap<Value, V>;

use crate::ast::{Statements, Statement, Value, LogKind, Integer};

pub struct RunOutput {
    pub stdout: String,
    pub variables: SpeckyDataContainer<Value>,
    pub jump_addresses: SpeckyDataContainer<usize>,
}

pub fn run(parsed: &Statements) -> RunOutput {
    let mut variables: SpeckyDataContainer<Value> = SpeckyDataContainer::default();
    let mut jump_addresses: SpeckyDataContainer<usize> = SpeckyDataContainer::default();
    let mut current_pointer: Value = Value::Null;

    let mut line_index = 0;

    let mut output = String::new();
    loop {
        if line_index >= parsed.len() { break; }

        macro_rules! match_statement {
            { $($statement:ident $($expr:tt)? => $code:tt $(,)?)* } => {
                match &parsed[line_index] {
                    $(
                        match_statement!(@pat $statement $($expr)?) => {
                            $(
                                #[allow(unused_macros)]
                                macro_rules! operand {
                                    () => {
                                        {
                                            let mut final_value = &$expr.value;
                                            for _ in 0..$expr.reader {
                                                final_value = variables.get(&$expr.value).unwrap_or(&Value::Null)
                                            }
                                            final_value
                                        }
                                    };
                                }
                            )?
            
                            #[allow(unused_macros)]
                            macro_rules! left_right_operator {
                                ($callback:expr) => {
                                    let left = variables.get(&current_pointer).unwrap_or(&Value::Null).clone();
                                    let right = match operand!() {
                                        Value::Symbol(_) => variables.get(operand!()).unwrap_or(&Value::Null).clone(),
                                        _ => operand!().clone(),
                                    };
                                    #[allow(clippy::redundant_closure_call)]
                                    let result = $callback(left, right);
                                    variables.insert(current_pointer.clone(), result);
                                }
                            }

                            $code
                        },
                    )*
                }
            };
            (@pat $ident:ident $(())?) => { Statement::$ident };
            (@pat $ident:ident $expr:tt) => { Statement::$ident $expr };
        }

        match_statement! {
            Load(expr) => {
                current_pointer = operand!().clone();
            },
            Define(expr) => {
                jump_addresses.insert(operand!().clone(), line_index);
            },
            Jump(expr) => {
                if let Some(index) = jump_addresses.get(operand!()) {
                    line_index = *index
                }
            },
            Assign(expr) => {
                variables.insert(
                    current_pointer.clone(),
                    match operand!().clone() {
                        Value::Time(_) => Value::Time(Instant::now()),
                        rest => rest,
                    }
                );
            },
            Overwrite(expr) => {
                variables.insert(operand!().clone(), current_pointer.clone());
            },
            Swap(expr) => {
                let temp = variables.get(&current_pointer).unwrap_or(&Value::Null).clone();
                variables.insert(current_pointer.clone(), variables.get(operand!()).unwrap_or(&Value::Null).clone());
                variables.insert(operand!().clone(), temp);
            },
            And(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (true, true) => Value::Boolean(true),
                        _ => Value::Boolean(false),
                    }
                });
            },
            Or(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (false, false) => Value::Boolean(false),
                        _ => Value::Boolean(true),
                    }
                });
            },
            Xor(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (true, false)|(false, true) => Value::Boolean(true),
                        _ => Value::Boolean(false),
                    }
                });
            },
            Plus(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + &right),
                        (Value::Text(left), Value::Text(right)) => Value::Text(left + &right),
                        _ => Value::Null,
                    }
                });
            },
            Minus(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - &right),
                        _ => Value::Null,
                    }
                });
            },
            Times(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * &right),
                        _ => Value::Null,
                    }
                });
            },
            Divide(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left / &right),
                        _ => Value::Null,
                    }
                });
            },
            Modulo(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left % &right),
                        _ => Value::Null,
                    }
                });
            },
            Exponential(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Integer(left.pow(right.try_into().unwrap())),
                        _ => Value::Null,
                    }
                });
            },
            Unequal(expr) => {
                left_right_operator!(|left, right| Value::Boolean(left != right));
            },
            Equal(expr) => {
                left_right_operator!(|left, right| Value::Boolean(left == right));
            },
            LessThan(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left < right),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left < right),
                        _ => Value::Null,
                    }
                });
            },
            GreaterThan(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left > right),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left > right),
                        _ => Value::Null,
                    }
                });
            },
            LessThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left <= right),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left <= right),
                        _ => Value::Null,
                    }
                });
            },
            GreaterThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left >= right),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left >= right),
                        _ => Value::Null,
                    }
                });
            },
            Truthy(quantity) => {
                let value = variables.get(&current_pointer).unwrap_or(&Value::Null);
                if !value_is_truthy(value) {
                    line_index += quantity;
                }
            },
            Falsy(quantity) => {
                let value = variables.get(&current_pointer).unwrap_or(&Value::Null);
                if value_is_truthy(value) {
                    line_index += quantity;
                }
            },
            Exists(quantity) => {
                let value = variables.get(&current_pointer).unwrap_or(&Value::Null);
                if !value_exists(value) {
                    line_index += quantity; 
                }
            },
            Empty(quantity) => {
                let value = variables.get(&current_pointer).unwrap_or(&Value::Null);
                if value_exists(value) {
                    line_index += quantity;
                }
            },
            Log { kind, reader, reverse, newline, space, vertical } => {
                let mut print = match kind {
                    LogKind::Value => variables.get(&current_pointer).unwrap_or(&Value::Null),
                    LogKind::Pointer => &current_pointer,
                };

                for _ in 0..*reader {
                    print = variables.get(&current_pointer).unwrap_or(&Value::Null);
                }

                let string = value_to_string(print).to_string();

                let string = if *reverse {
                    string.chars().rev().collect()
                } else {
                    string
                };

                let string = string + &" ".repeat(*space);
                let string = string + if *newline { "\n" } else { "" };

                let string = if *vertical {
                    let max = string.lines().map(|l| l.len()).max().unwrap_or(0);
                    let lines = string.lines().collect::<Vec<&str>>();
                    let mut output_lines: Vec<String> = vec![];

                    for i in 0..max {
                        output_lines.push(lines.iter().map(|l| l.chars().nth(i).unwrap_or(' ')).collect());
                    }
                    output_lines.join("\n")
                } else {
                    string
                };

                print!("{string}");

                output.push_str(&string);
            },
        }

        line_index += 1;
    }

    RunOutput {
        stdout: output,
        variables,
        jump_addresses,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Symbol(s) => s.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Text(s) => format!("/{}/", s.replace('/', r"\/")),
        Value::Time(d) => format!("{:?}", d.elapsed()),
        Value::Null => "null".to_string(),
    }
}

fn value_is_truthy(value: &Value) -> bool {
    match value {
        Value::Symbol(_) => true,
        Value::Boolean(b) => *b,
        Value::Integer(n) => *n != Integer::from(0),
        Value::Text(s) => !s.is_empty(),
        Value::Time(_) => true,
        Value::Null => false,
    }
}

fn value_exists(value: &Value) -> bool {
    !matches!(value, Value::Null)
}
