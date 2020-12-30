use parser::ast::{Expression, InfixOperation};

use super::{env::Environment, eval_expression, object::Object, EvaluationError};

fn eval_sum_expression(
    env: &mut Environment,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(env, left)?;
    let right_v = eval_expression(env, right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a + b)),
        (left_v, right_v) => Err(EvaluationError::InfixOperationNotImplemented {
            operation: InfixOperation::Sum,
            left: left_v,
            right: right_v,
        }),
    }
}

fn eval_product_expression(
    env: &mut Environment,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(env, left)?;
    let right_v = eval_expression(env, right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a * b)),
        (left_v, right_v) => Err(EvaluationError::InfixOperationNotImplemented {
            operation: InfixOperation::Product,
            left: left_v,
            right: right_v,
        }),
    }
}

fn eval_subtract_expression(
    env: &mut Environment,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(env, left)?;
    let right_v = eval_expression(env, right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a - b)),
        (left_v, right_v) => Err(EvaluationError::InfixOperationNotImplemented {
            operation: InfixOperation::Subtraction,
            left: left_v,
            right: right_v,
        }),
    }
}

fn eval_division_expression(
    env: &mut Environment,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(env, left)?;
    let right_v = eval_expression(env, right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a / b)),
        (left_v, right_v) => Err(EvaluationError::InfixOperationNotImplemented {
            operation: InfixOperation::Division,
            left: left_v,
            right: right_v,
        }),
    }
}

pub fn eval(
    env: &mut Environment,
    operation: &InfixOperation,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    match operation {
        InfixOperation::Sum => eval_sum_expression(env, left, right),
        InfixOperation::Product => eval_product_expression(env, left, right),
        InfixOperation::Subtraction => eval_subtract_expression(env, left, right),
        InfixOperation::Division => eval_division_expression(env, left, right),
        _ => panic!("Not implemented"),
    }
}
