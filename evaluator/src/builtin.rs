use std::cell::RefCell;
use std::rc::Rc;

use super::{env::Environment, object::Object, EvaluationError};

pub fn eval_builtin_call(arg_values: Vec<Object>, name: String) -> Result<Object, EvaluationError> {
    match name.as_ref() {
        "len" => match &arg_values[..] {
            [Object::Str(v)] => Ok(Object::Integer(v.len() as i64)),
            [Object::Array(a)] => Ok(Object::Integer(a.len() as i64)),
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "string or array",
            }),
        },
        "first" => match &arg_values[..] {
            [Object::Array(a)] => {
                if a.is_empty() {
                    return Err(EvaluationError::IndexOutOfBounds {
                        value: Object::Array(a.clone()),
                        index: 0,
                    });
                }
                Ok(a[0].clone())
            }
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "array",
            }),
        },
        "last" => match &arg_values[..] {
            [Object::Array(a)] => {
                if a.is_empty() {
                    return Err(EvaluationError::IndexOutOfBounds {
                        value: Object::Array(a.clone()),
                        index: 0,
                    });
                }
                Ok(a[a.len() - 1].clone())
            }
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "array",
            }),
        },
        "rest" => match &arg_values[..] {
            [Object::Array(a)] => {
                if a.len() < 2 {
                    return Err(EvaluationError::IndexOutOfBounds {
                        value: Object::Array(a.clone()),
                        index: 0,
                    });
                }
                Ok(Object::Array(a[1..].to_owned()))
            }
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "array",
            }),
        },

        "push" => match &arg_values[..] {
            [Object::Array(a), o] => {
                let mut b = a.clone();
                b.push(o.clone());
                Ok(Object::Array(b))
            }
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "(array, object)",
            }),
        },

        "puts" => {
            for (index, item) in arg_values.iter().enumerate() {
                print!("{}", item);
                if index < arg_values.len() - 1 {
                    print!(" ");
                }
            }

            println!();

            Ok(Object::Null)
        }
        n => panic!("Unknown builtin function {}", n),
    }
}

pub fn set_builtins_to_env(env: &Rc<RefCell<Environment>>) {
    Environment::set_rr(
        env,
        String::from("len"),
        Object::BuiltInFunction(String::from("len")),
    );

    Environment::set_rr(
        env,
        String::from("first"),
        Object::BuiltInFunction(String::from("first")),
    );

    Environment::set_rr(
        env,
        String::from("last"),
        Object::BuiltInFunction(String::from("last")),
    );

    Environment::set_rr(
        env,
        String::from("rest"),
        Object::BuiltInFunction(String::from("rest")),
    );

    Environment::set_rr(
        env,
        String::from("push"),
        Object::BuiltInFunction(String::from("push")),
    );

    Environment::set_rr(
        env,
        String::from("puts"),
        Object::BuiltInFunction(String::from("puts")),
    );
}
