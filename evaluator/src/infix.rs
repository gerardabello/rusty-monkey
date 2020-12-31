use std::cell::RefCell;
use std::rc::Rc;

use parser::ast::{Expression, InfixOperation};

use super::{env::Environment, eval_expression, object::Object, EvaluationError};

type InfixFn = fn(&Object, &Object) -> Option<Object>;

fn sum(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Integer(a + b)),
        _ => None,
    }
}

fn subtraction(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Integer(a - b)),
        _ => None,
    }
}

fn division(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Integer(a / b)),
        _ => None,
    }
}

fn product(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Integer(a * b)),
        _ => None,
    }
}

fn equal(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a == b)),
        (Object::Bool(a), Object::Bool(b)) => Some(Object::Bool(a == b)),
        _ => None,
    }
}

fn not_equal(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a != b)),
        (Object::Bool(a), Object::Bool(b)) => Some(Object::Bool(a != b)),
        _ => None,
    }
}

fn less_than(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a < b)),
        _ => None,
    }
}

fn greater_than(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a > b)),
        _ => None,
    }
}

fn less_than_equal(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a <= b)),
        _ => None,
    }
}

fn greater_than_equal(a: &Object, b: &Object) -> Option<Object> {
    match (a, b) {
        (Object::Integer(a), Object::Integer(b)) => Some(Object::Bool(a >= b)),
        _ => None,
    }
}

pub fn eval(
    env: &Rc<RefCell<Environment>>,
    operation: &InfixOperation,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let func: InfixFn = match operation {
        InfixOperation::Sum => sum,
        InfixOperation::Subtraction => subtraction,
        InfixOperation::Division => division,
        InfixOperation::Product => product,
        InfixOperation::Equal => equal,
        InfixOperation::LessThan => less_than,
        InfixOperation::GreaterThan => greater_than,
        InfixOperation::LessThanEqual => less_than_equal,
        InfixOperation::GreaterThanEqual => greater_than_equal,
        InfixOperation::NotEqual => not_equal,
    };

    let left_v = eval_expression(env, left)?;
    let right_v = eval_expression(env, right)?;

    match func(&left_v, &right_v) {
        Some(v) => Ok(v),
        None => Err(EvaluationError::InfixOperationNotImplemented {
            operation: operation.clone(),
            left: left_v,
            right: right_v,
        }),
    }
}
