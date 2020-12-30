use parser::ast::{Expression, Statement};

use super::{eval_expression, eval_statements, object::Object, EvaluationError};

pub fn eval(
    condition: &Expression,
    consequence: &[Statement],
    alternative: &Option<Vec<Statement>>,
) -> Result<Object, EvaluationError> {
    match eval_expression(condition)? {
        Object::Bool(true) => eval_statements(consequence),
        Object::Bool(false) => match alternative {
            None => Ok(Object::Null),
            Some(alt) => eval_statements(alt),
        },
        v => Err(EvaluationError::UnexpectedType {
            value: v,
            expected: "bool",
        }),
    }
}
