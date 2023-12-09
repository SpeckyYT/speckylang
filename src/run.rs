use std::time::{Instant, Duration};
use std::io::{self, Write};

use fnv::FnvHashMap;

type SpeckyDataContainer<V> = FnvHashMap<Value, V>;

use crate::ast::{Statements, Statement, Value, LogKind, Integer, Float, SmallInt};

const NULL: Value = Value::Null;

#[derive(Debug)]
pub struct RunOutput {
    pub stdout: String,
    pub variables: SpeckyDataContainer<Value>,
}

pub fn run(parsed: &Statements) -> RunOutput {
    let mut variables: SpeckyDataContainer<Value> = SpeckyDataContainer::default();
    let mut current_pointer: Value = Value::Null;

    let mut line_index = 0;

    let mut max_time: (Duration, Statement) = (Duration::ZERO, Statement::Truthy(0));

    let mut output = String::new();
    loop {
        if line_index >= parsed.len() { break; }

        let mut changed_pointer = false;

        macro_rules! match_statement {
            { $($statement:ident $($expr:tt)? => $code:tt $(,)?)* } => {
                match &parsed[line_index] {
                    $(
                        match_statement!(@pat $statement $($expr)?) => {
                            $(
                                #[allow(unused_macros)]
                                macro_rules! operand {
                                    () => {
                                        value_reader(&variables, &$expr.value, $expr.reader).clone()
                                    };
                                }

                                #[allow(unused_macros)]
                                macro_rules! condition_jump {
                                    ($condition:expr, $quantity:expr) => {
                                        {
                                            let value = variables.get(&current_pointer).unwrap_or(&Value::Null);
                                            #[allow(clippy::redundant_closure)]
                                            #[allow(clippy::redundant_closure_call)]
                                            if !$condition(value) { line_index += $quantity; }
                                        }
                                    }
                                }
                            )?
            
                            #[allow(unused_macros)]
                            macro_rules! left_right_operator {
                                ($callback:expr) => {
                                    let current_operand = operand!();
                                    let left = variables.get(&current_pointer).unwrap_or(&Value::Null).clone();
                                    let right = match current_operand {
                                        Value::Symbol(_) => variables.get(&current_operand).unwrap_or(&Value::Null).clone(),
                                        _ => current_operand.clone(),
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

        let start_operation = Instant::now();

        match_statement! {
            Load(expr) => {
                current_pointer = operand!().clone();
                changed_pointer = true;
            },
            Define(expr) => {
                variables.insert(operand!().clone(), Value::JumpAddress(line_index));
            },
            Jump(expr) => {
                if let Some(Value::JumpAddress(index)) = variables.get(&operand!()) {
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
                let current_operand = operand!();
                let temp = variables.get(&current_pointer).unwrap_or(&Value::Null).clone();
                variables.insert(current_pointer.clone(), variables.get(&current_operand).unwrap_or(&Value::Null).clone());
                variables.insert(current_operand.clone(), temp);
            },
            Index(expr) => {
                left_right_operator!(|left, right| {
                    match (left, right) {
                        (Value::Text(string), right) => {
                            let integer: Option<usize> = match right {
                                Value::Integer(int) => int.try_into().ok(),
                                Value::SmallInt(int) => int.try_into().ok(),
                                _ => return Value::Null,
                            };
                            if let Some(int) = integer {
                                if let Some(ch) = string.chars().nth(int) {
                                    Value::Text(ch.to_string())
                                } else {
                                    Value::Null
                                }
                            } else {
                                Value::Null
                            }
                            
                        }
                        _ => Value::Null,
                    }
                });
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
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left + &right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::SmallInt(left + &right),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left + Integer::from(right)),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left) + &right),
                        (Value::Text(left), Value::Text(right)) => Value::Text(left + &right),
                        (Value::Text(left), Value::Integer(right)) => Value::Text(left + &right.to_string()),
                        (Value::Text(left), Value::SmallInt(right)) => Value::Text(left + &right.to_string()),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left + right),
                        _ => Value::Null,
                    }
                });
            },
            Minus(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left - &right),
                        (Value::SmallInt(left), Value::SmallInt(right)) =>
                            left.checked_sub(right)
                            .map(Value::SmallInt)
                            .unwrap_or(Value::Integer(Integer::from(left) - Integer::from(right))),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left - Integer::from(right)),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left) - right),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
                        _ => Value::Null,
                    }
                });
            },
            Times(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left * &right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => left.checked_mul(right)
                            .map(Value::SmallInt)
                            .unwrap_or(Value::Integer(Integer::from(left) * Integer::from(right))),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left) * right),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left * Integer::from(right)),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
                        _ => Value::Null,
                    }
                });
            },
            Divide(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left / &right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::SmallInt(left / right),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left / &Integer::from(right)),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left) / &right),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left / right),
                        _ => Value::Null,
                    }
                });
            },
            Modulo(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left % &right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::SmallInt(left % right),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left % &Integer::from(right)),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left) % &right),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left % right),
                        _ => Value::Null,
                    }
                });
            },
            Exponential(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => compress_integer(left.pow(right.try_into().unwrap())),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::SmallInt(left.pow(right.try_into().unwrap())),
                        (Value::Integer(left), Value::SmallInt(right)) => compress_integer(left.pow(right.try_into().unwrap())),
                        (Value::SmallInt(left), Value::Integer(right)) => compress_integer(Integer::from(left).pow(right.try_into().unwrap())),
                        (Value::Float(left), Value::Float(right)) => Value::Float(left.pow(&right)),
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
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::Boolean(left < right),
                        (Value::SmallInt(left), Value::Integer(right)) => Value::Boolean(Integer::from(left) < right),
                        (Value::Integer(left), Value::SmallInt(right)) => Value::Boolean(left < Integer::from(right)),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left < right),
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left < right),
                        _ => Value::Null,
                    }
                });
            },
            GreaterThan(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left > right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::Boolean(left > right),
                        (Value::SmallInt(left), Value::Integer(right)) => Value::Boolean(Integer::from(left) > right),
                        (Value::Integer(left), Value::SmallInt(right)) => Value::Boolean(left > Integer::from(right)),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left > right),
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left > right),
                        _ => Value::Null,
                    }
                });
            },
            LessThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left <= right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::Boolean(left <= right),
                        (Value::SmallInt(left), Value::Integer(right)) => Value::Boolean(Integer::from(left) <= right),
                        (Value::Integer(left), Value::SmallInt(right)) => Value::Boolean(left <= Integer::from(right)),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left <= right),
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
                        _ => Value::Null,
                    }
                });
            },
            GreaterThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left >= right),
                        (Value::SmallInt(left), Value::SmallInt(right)) => Value::Boolean(left >= right),
                        (Value::SmallInt(left), Value::Integer(right)) => Value::Boolean(Integer::from(left) >= right),
                        (Value::Integer(left), Value::SmallInt(right)) => Value::Boolean(left >= Integer::from(right)),
                        (Value::Text(left), Value::Text(right)) => Value::Boolean(left >= right),
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left >= right),
                        _ => Value::Null,
                    }
                });
            },
            Truthy(quantity) => { condition_jump!(|value| value_is_truthy(value), quantity); },
            Falsy(quantity) => { condition_jump!(|value| !value_is_truthy(value), quantity); },
            Exists(quantity) => { condition_jump!(|value| value_exists(value), quantity); },
            Empty(quantity) => { condition_jump!(|value| !value_exists(value), quantity); },
            Log { kind, reader, special, reverse, newline, space, vertical, assign } => {
                let string = match kind {
                    Some(LogKind::Pointer)|Some(LogKind::Value) => {
                        let reader = kind.unwrap().reader_count() + reader;
                        let print = value_reader(&variables, &current_pointer, reader);
                        value_to_string(print, *special)
                    },
                    Some(LogKind::Type) => {
                        match variables.get(&current_pointer).unwrap_or(&NULL) {
                            Value::JumpAddress(_) => "JumpAddress",
                            Value::Symbol(_) => "Symbol",
                            Value::Boolean(_) => "Boolean",
                            Value::Integer(_) => "Integer",
                            Value::SmallInt(_) => "SmallInt",
                            Value::Float(_) => "Float",
                            Value::Text(_) => "Text",
                            Value::Time(_) => "Time",
                            Value::Null => "Null",
                        }.to_string()
                    },
                    None => "".to_string(),
                };

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

                if *assign {
                    let _ = variables.insert(
                        current_pointer.clone(),
                        match string.as_str() {
                            "false" => Value::Boolean(false),
                            "true" => Value::Boolean(true),
                            "null" => Value::Null,
                            string if string.chars().all(|c| char::is_ascii_digit(&c)) =>
                                Value::Integer(string.parse().unwrap()),
                            string => Value::Text(string.to_string()),
                        }
                    );
                }

                print!("{string}");

                output.push_str(&string);
            },
            Input() => {
                variables.insert(current_pointer.clone(), value_input());
            },
        }

        if start_operation.elapsed() > max_time.0 {
            max_time = (start_operation.elapsed(), parsed[line_index].clone())
        }

        if changed_pointer { compress_value(&mut current_pointer) }

        line_index += 1;
    }
    
    // println!("he be named max and doing the minimum: {:?}", max_time);

    RunOutput {
        stdout: output,
        variables,
    }
}

#[inline(always)]
fn compress_value(value: &mut Value) {
    if let Value::Integer(int) = &value {
        let si = int.try_into();
        if let Ok(si) = si { 
            *value = Value::SmallInt(si);
        }
    };
}

#[inline(always)]
fn compress_integer(integer: Integer) -> Value {
    (&integer).try_into()
    .map(Value::SmallInt)
    .unwrap_or(Value::Integer(integer))
}

fn value_input() -> Value {
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap_or(0);
    string_to_value(&s)
}

fn string_to_value(string: &str) -> Value {
    let string = string.trim();

    if string.is_empty() {
        return Value::Null
    }

    if string.chars().all(|c| char::is_ascii_digit(&c)) {
        return string.parse::<SmallInt>()
            .map(Value::SmallInt)
            .unwrap_or(Value::Integer(string.parse::<Integer>().unwrap()))
    }

    if string.chars().filter(|c| c == &'.').count() == 2 {
        return Value::Float(string.parse::<Float>().unwrap())
    }

    return match string {
        "true"|"on"|"yes" => Value::Boolean(true),
        "false"|"off"|"no" => Value::Boolean(false),
        "null" => Value::Null,
        "µ" => Value::Time(Instant::now()),
        string if string.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') =>
            Value::Symbol(string.to_string()),
        string => Value::Text(string.to_string()),
    };
}

fn value_reader<'a>(memory: &'a SpeckyDataContainer<Value>, value: &'a Value, reader: usize) -> &'a Value {
    let mut chain: Vec<&Value> = vec![value];

    let mut current_value = value;

    for i in 0..reader {
        let temp_value: Value;

        let current_pointer = match current_value {
            Value::SmallInt(int) => {
                temp_value = Value::Integer(Integer::from(*int));
                if memory.contains_key(current_value) {
                    current_value
                } else {
                    &temp_value
                }
            },
            Value::Integer(int) => {
                let small_form = int.try_into()
                .map(Value::SmallInt);

                if small_form.is_ok() && memory.contains_key(small_form.as_ref().unwrap()) {
                    temp_value = small_form.unwrap();
                    &temp_value
                } else {
                    current_value
                }
            },
            other => other,
        };

        current_value = memory.get(current_pointer).unwrap_or(&NULL);

        let exists = chain.iter().enumerate().find_map(|(i, v)| if v == &current_value { Some(i) } else { None });

        if let Some(index) = exists {
            // println!("{chain:?} ({current_value:?}) [{reader} | {i}]");

            let base = reader + i + index + 1;
            let modulo = chain.len() - index;
            let chain_index = base % modulo + index;

            return chain[chain_index];
        }

        chain.push(current_value);
    }

    current_value
}

fn value_to_string(value: &Value, special: bool) -> String {
    match (value, special) {
        (Value::Symbol(s), false) => s.to_string(),
        (Value::Symbol(s), true) => Integer::from_bytes_be(num_bigint::Sign::Plus, s.as_bytes()).to_string(),

        (Value::Boolean(b), false) => b.to_string(),
        (Value::Boolean(b), true) => format!("{}", if *b { 1 } else { 0 }),

        (Value::Integer(i), false) => i.to_string(),
        (Value::Integer(i), true) =>
            i.try_into()
            .ok()
            .and_then(char::from_u32)
            .map(|c| c.to_string())
            .unwrap_or_else(|| char::REPLACEMENT_CHARACTER.to_string()),

        (Value::SmallInt(i), false) => i.to_string(),
        (Value::SmallInt(i), true) =>
            (*i as u128).try_into()
            .ok()
            .and_then(char::from_u32)
            .map(|c| c.to_string())
            .unwrap_or_else(|| char::REPLACEMENT_CHARACTER.to_string()),

        (Value::Float(f), false) => f.to_string(),
        (Value::Float(f), true) => f.to_f64().to_string(),

        (Value::Text(s), false) => format!("/{}/", s.replace('/', r"\/")),
        (Value::Text(s), true) => s.to_string(),

        (Value::Time(d), false) => format!("{:?}", d.elapsed()),
        (Value::Time(d), true) => format!("{}", d.elapsed().as_secs_f64()),

        (Value::Null, false) => "null".to_string(),
        (Value::Null, true) => "\0".to_string(),

        (Value::JumpAddress(a), false) => a.to_string(),
        (Value::JumpAddress(a), true) => format!("{:b}", a),
    }
}

fn value_is_truthy(value: &Value) -> bool {
    match value {
        Value::Symbol(_) => true,
        Value::Boolean(b) => *b,
        Value::Integer(n) => *n != Integer::from(0),
        Value::SmallInt(i) => *i != 0,
        Value::Float(f) => !f.is_nan() && !f.is_inf() && *f != Float::from(0.0),
        Value::Text(s) => !s.is_empty(),
        Value::Time(_) => true,
        Value::Null => false,
        Value::JumpAddress(_) => true
    }
}

fn value_exists(value: &Value) -> bool {
    !matches!(value, Value::Null)
}
