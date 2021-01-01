use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{env::Environment, eval_expression, eval_statements, object::Object, EvaluationError};

pub fn eval_builtin_call(arg_values: Vec<Object>, name: String) -> Result<Object, EvaluationError> {
    match name.as_ref() {
        "len" => match &arg_values[..] {
            [Object::Str(v)] => Ok(Object::Integer(v.len() as i64)),
            _ => Err(EvaluationError::InvalidArguments {
                values: arg_values,
                expected: "string",
            }),
        },
        n => panic!("Unknown builtin function {}", n),
    }
}

pub fn set_builtins_to_env(env: &Rc<RefCell<Environment>>) {
    Environment::set_rr(
        env,
        String::from("len"),
        Object::BuiltInFunction(String::from("len")),
    );
}
