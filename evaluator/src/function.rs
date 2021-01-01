use std::cell::RefCell;
use std::rc::Rc;

use parser::ast::{Expression, Statement};

use super::{
    builtin::eval_builtin_call, env::Environment, eval_expression, eval_statements, object::Object,
    EvaluationError,
};

pub fn eval_call(
    env: &Rc<RefCell<Environment>>,
    function: &Expression,
    arguments: &[Expression],
) -> Result<Object, EvaluationError> {
    let function_value = eval_expression(env, function)?;

    let arg_values = arguments
        .iter()
        .flat_map(|a| eval_expression(env, a))
        .collect::<Vec<_>>();

    match function_value {
        Object::Function(arg_names, body, clojure) => {
            eval_monkey_call(arg_values, arg_names, body, clojure)
        }
        Object::BuiltInFunction(name) => eval_builtin_call(arg_values, name),
        _ => Err(EvaluationError::NotCallable {
            value: function_value.clone(),
        }),
    }
}

fn eval_monkey_call(
    arg_values: Vec<Object>,
    arg_names: Vec<String>,
    body: Vec<Statement>,
    clojure: Rc<RefCell<Environment>>,
) -> Result<Object, EvaluationError> {
    let new_env = Rc::new(RefCell::new(Environment::with_parent(&clojure)));

    for (k, v) in arg_names.iter().zip(arg_values) {
        Environment::set_rr(&new_env, k.to_owned(), v);
    }

    eval_statements(&new_env, &body)
}

pub fn eval_function(
    env: &Rc<RefCell<Environment>>,
    arguments: &[String],
    body: &[Statement],
) -> Result<Object, EvaluationError> {
    Ok(Object::Function(
        arguments.to_owned(),
        body.to_owned(),
        Rc::clone(env),
    ))
}
