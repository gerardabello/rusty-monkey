use parser::ast::{Expression, InfixOperation};

use super::{eval_expression, object::Object, EvaluationError};

fn eval_sum_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

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
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

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
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

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
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

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
    operation: &InfixOperation,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    match operation {
        InfixOperation::Sum => eval_sum_expression(left, right),
        InfixOperation::Product => eval_product_expression(left, right),
        InfixOperation::Subtraction => eval_subtract_expression(left, right),
        InfixOperation::Division => eval_division_expression(left, right),
        _ => panic!("Not implemented"),
    }
}
