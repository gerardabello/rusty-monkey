use parser::ast::{Expression, Statement};

use super::{eval_expression, eval_statements, object::Object, EvaluationError, env::Environment};

pub fn eval(
    env: &mut Environment,
    condition: &Expression,
    consequence: &[Statement],
    alternative: &Option<Vec<Statement>>,
) -> Result<Object, EvaluationError> {
    match eval_expression(env, condition)? {
        Object::Bool(true) => eval_statements(env, consequence),
        Object::Bool(false) => match alternative {
            None => Ok(Object::Null),
            Some(alt) => eval_statements(env, alt),
        },
        v => Err(EvaluationError::UnexpectedType {
            value: v,
            expected: "bool",
        }),
    }
}
