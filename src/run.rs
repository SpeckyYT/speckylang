use fnv::FnvHashMap;

type SpeckyHashMap<K, V> = FnvHashMap<K, V>;

use crate::ast;

pub fn run(parsed: ast::Program) -> String {
    let mut variables: SpeckyHashMap<ast::Value, ast::Value> = SpeckyHashMap::default();
    let mut jump_stack: SpeckyHashMap<ast::Value, usize> = SpeckyHashMap::default();
    let mut current_address: ast::Value = ast::Value::Null;

    let mut line_index = 0;

    let mut output = String::new();

    loop {
        if line_index >= parsed.len() { break; }
        
        use ast::Operator::*;

        match &parsed[line_index] {
            ast::Operation::Dual(operator, reader, operand) => {
                macro_rules! operand {
                    () => {
                        {
                            let mut final_value = operand;
                            for _ in 0..*reader {
                                final_value = variables.get(operand).unwrap_or(&ast::Value::Null)
                            }
                            final_value
                        }
                    };
                }

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

                match operator {
                    Load => {
                        current_address = operand!().clone();
                    },
                    Define => {
                        jump_stack.insert(operand!().clone(), line_index);
                    },
                    Jump => {
                        match jump_stack.get(operand!()) {
                            Some(index) => line_index = *index,
                            None => (),
                        }
                    },
                    Assign => {
                        variables.insert(current_address.clone(), operand!().clone());
                    },
                    Overwrite => {
                        variables.insert(operand!().clone(), current_address.clone());
                    },
                    Swap => {
                        let temp = variables.get(&current_address).unwrap_or(&ast::Value::Null).clone();
                        variables.insert(current_address.clone(), variables.get(operand!()).unwrap_or(&ast::Value::Null).clone());
                        variables.insert(operand!().clone(), temp);
                    },
                    And => {
                        left_right_operator!(|left, right|{
                            match (value_is_truthy(&left), value_is_truthy(&right)) {
                                (true, true) => ast::Value::Boolean(true),
                                _ => ast::Value::Boolean(false),
                            }
                        });
                    },
                    Or => {
                        left_right_operator!(|left, right|{
                            match (value_is_truthy(&left), value_is_truthy(&right)) {
                                (false, false) => ast::Value::Boolean(false),
                                _ => ast::Value::Boolean(true),
                            }
                        });
                    },
                    Xor => {
                        left_right_operator!(|left, right|{
                            match (value_is_truthy(&left), value_is_truthy(&right)) {
                                (true, false)|(false, true) => ast::Value::Boolean(true),
                                _ => ast::Value::Boolean(false),
                            }
                        });
                    },
                    Plus => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left + &right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::String(left + &right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Minus => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left - &right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Times => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left * &right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Divide => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left / &right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Modulo => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left % &right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Exponential => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Integer(left.pow(right.try_into().unwrap())),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    Unequal => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left != right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left != right),
                                _ => ast::Value::Boolean(true),
                            }
                        });
                    },
                    Equal => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left == right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left == right),
                                _ => ast::Value::Boolean(false),
                            }
                        });
                    },
                    LessThan => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left < right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left < right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    GreaterThan => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left > right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left > right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    LessThanOrEqual => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left <= right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left <= right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    GreaterThanOrEqual => {
                        left_right_operator!(|left, right|{
                            match (left, right) {
                                (ast::Value::Integer(left), ast::Value::Integer(right)) => ast::Value::Boolean(left >= right),
                                (ast::Value::String(left), ast::Value::String(right)) => ast::Value::Boolean(left >= right),
                                _ => ast::Value::Null,
                            }
                        });
                    },
                    _ => todo!(),
                }
            },
            ast::Operation::Mono(operator) => {
                match operator {
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
                        let string = format!(
                            "{}",
                            value_to_string(variables.get(&current_address).unwrap_or(&ast::Value::Null)),
                        );
                        println!("{string}");
                        output.push_str(&(string + "\n"));
                    },
                    LogCurrentAddress => {
                        let string = format!(
                            "{}",
                            value_to_string(variables.get(&current_address).unwrap_or(&ast::Value::Null)),
                        );
                        println!("{string}");
                        output.push_str(&(string + "\n"));
                    },
                    _ => todo!(),
                }
            },
        }

        line_index += 1;
    }

    output
}

fn value_to_string(value: &ast::Value) -> String {
    use ast::Value::*;
    match value {
        Symbol(s) => format!("{}", s),
        Boolean(b) => format!("{}", b),
        Integer(i) => format!("{}", i),
        String(s) => format!("/{}/", s.replace("/", r"\/")),
        Null => "null".to_string(),
    }
}

fn value_is_truthy(value: &ast::Value) -> bool {
    use ast::Value::*;
    match value {
        Symbol(_) => true,
        Boolean(b) => *b,
        Integer(n) => *n != crate::ast::Integer::from(0),
        String(s) => s.len() > 0,
        Null => false,
    }
}

fn value_exists(value: &ast::Value) -> bool {
    use ast::Value::*;
    match value {
        Null => false,
        _ => true,
    }
}
