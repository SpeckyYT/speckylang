use std::time::Instant;

use fnv::FnvHashMap;

type SpeckyDataContainer<V> = FnvHashMap<ast::Value, V>;

use crate::ast::{self, Statement};

pub struct RunOutput {
    pub stdout: String,
    pub variables: SpeckyDataContainer<ast::Value>,
    pub jump_stack: SpeckyDataContainer<usize>,
}

pub fn run(parsed: &ast::Statements) -> RunOutput {
    let mut variables: SpeckyDataContainer<ast::Value> = SpeckyDataContainer::default();
    let mut jump_stack: SpeckyDataContainer<usize> = SpeckyDataContainer::default();
    let mut current_address: ast::Value = ast::Value::Null;

    let mut line_index = 0;

    let mut output = String::new();

    loop {
        if line_index >= parsed.len() { break; }

        macro_rules! match_statement {
            {$($statement:ident $(($expr:ident))? => $code:tt $(,)?)*} => {
                match &parsed[line_index] {
                    $(
                        Statement::$statement $(($expr))? => {

                            $(
                                #[allow(unused_macros)]
                                macro_rules! operand {
                                    () => {
                                        {
                                            let mut final_value = &$expr.value;
                                            for _ in 0..$expr.reader {
                                                final_value = variables.get(&$expr.value).unwrap_or(&ast::Value::Null)
                                            }
                                            final_value
                                        }
                                    };
                                }
                            )?
            
                            #[allow(unused_macros)]
                            macro_rules! left_right_operator {
                                ($callback:expr) => {
                                    let left = variables.get(&current_address).unwrap_or(&ast::Value::Null).clone();
                                    let right = match operand!() {
                                        ast::Value::Symbol(_) => variables.get(operand!()).unwrap_or(&ast::Value::Null).clone(),
                                        _ => operand!().clone(),
                                    };
                                    let result = $callback(left, right);
                                    variables.insert(current_address.clone(), result);
                                }
                            }

                            $code
                        },
                    )*
                }
            };
        }

        match_statement!{
            Load(expr) => {
                current_address = operand!().clone();
            },
            Define(expr) => {
                jump_stack.insert(operand!().clone(), line_index);
            },
            Jump(expr) => {
                if let Some(index) = jump_stack.get(operand!()) {
                    line_index = *index
                }
            },
            Assign(expr) => {
                variables.insert(
                    current_address.clone(),
                    match operand!().clone() {
                        ast::Value::Time(_) => ast::Value::Time(Instant::now()),
                        rest => rest,
                    }
                );
            },
            Overwrite(expr) => {
                variables.insert(operand!().clone(), current_address.clone());
            },
            Swap(expr) => {
                let temp = variables.get(&current_address).unwrap_or(&ast::Value::Null).clone();
                variables.insert(current_address.clone(), variables.get(operand!()).unwrap_or(&ast::Value::Null).clone());
                variables.insert(operand!().clone(), temp);
            },
            And(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (true, true) => ast::Value::Boolean(true),
                        _ => ast::Value::Boolean(false),
                    }
                });
            },
            Or(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (false, false) => ast::Value::Boolean(false),
                        _ => ast::Value::Boolean(true),
                    }
                });
            },
            Xor(expr) => {
                left_right_operator!(|left, right|{
                    match (value_is_truthy(&left), value_is_truthy(&right)) {
                        (true, false)|(false, true) => ast::Value::Boolean(true),
                        _ => ast::Value::Boolean(false),
                    }
                });
            },
            Plus(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left + &right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::String(left + &right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Minus(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left - &right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Times(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left * &right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Divide(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left / &right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Modulo(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left % &right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Exponential(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left.pow(right.try_into().unwrap())),
                        _ => ast::Value::Null,
                    }
                });
            },
            Unequal(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left != right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left != right),
                        _ => ast::Value::Boolean(true),
                    }
                });
            },
            Equal(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left == right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left == right),
                        _ => ast::Value::Boolean(false),
                    }
                });
            },
            LessThan(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left < right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left < right),
                        _ => ast::Value::Null,
                    }
                });
            },
            GreaterThan(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left > right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left > right),
                        _ => ast::Value::Null,
                    }
                });
            },
            LessThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left <= right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left <= right),
                        _ => ast::Value::Null,
                    }
                });
            },
            GreaterThanOrEqual(expr) => {
                left_right_operator!(|left, right|{
                    match (left, right) {
                        (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left >= right),
                        (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left >= right),
                        _ => ast::Value::Null,
                    }
                });
            },
            Truthy => {
                let value = variables.get(&current_address).unwrap_or(&ast::Value::Null);
                if !value_is_truthy(value) {
                    line_index += 1;
                }
            },
            Falsy => {
                let value = variables.get(&current_address).unwrap_or(&ast::Value::Null);
                if value_is_truthy(value) {
                    line_index += 1;
                }
            },
            Exists => {
                let value = variables.get(&current_address).unwrap_or(&ast::Value::Null);
                if !value_exists(value) {
                    line_index += 1;
                }
            },
            Empty => {
                let value = variables.get(&current_address).unwrap_or(&ast::Value::Null);
                if value_exists(value) {
                    line_index += 1;
                }
            },
            LogValue => {
                let string = value_to_string(variables.get(&current_address).unwrap_or(&ast::Value::Null)).to_string();
                println!("{string}");
                output.push_str(&(string + "\n"));
            },
            LogCurrentAddress => {
                let string = value_to_string(&current_address).to_string();
                println!("{string}");
                output.push_str(&(string + "\n"));
            },
        }

        line_index += 1;
    }

    RunOutput {
        stdout: output,
        variables,
        jump_stack,
    }
}

fn value_to_string(value: &ast::Value) -> String {
    use ast::Value::*;
    match value {
        Symbol(s) => s.to_string(),
        Boolean(b) => b.to_string(),
        Integer(i) => i.to_string(),
        String(s) => format!("/{}/", s.replace('/', r"\/")),
        Time(d) => format!("{:?}", d.elapsed()),
        Null => "null".to_string(),
    }
}

fn value_is_truthy(value: &ast::Value) -> bool {
    use ast::Value::*;
    match value {
        Symbol(_) => true,
        Boolean(b) => *b,
        Integer(n) => *n != crate::ast::Integer::from(0),
        String(s) => !s.is_empty(),
        Time(_) => true,
        Null => false,
    }
}

fn value_exists(value: &ast::Value) -> bool {
    !matches!(value, ast::Value::Null)
}
