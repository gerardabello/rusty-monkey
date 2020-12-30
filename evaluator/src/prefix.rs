use parser::ast::{Expression, PrefixOperation};

use super::{eval_expression, object::Object, EvaluationError};

fn eval_negative(right: &Expression) -> Result<Object, EvaluationError> {
    let v = eval_expression(right)?;

    match v {
        Object::Integer(a) => Ok(Object::Integer(-a)),
        v => Err(EvaluationError::PrefixOperationNotImplemented {
            operation: PrefixOperation::Negative,
            right: v,
        }),
    }
}

fn eval_negate(right: &Expression) -> Result<Object, EvaluationError> {
    let v = eval_expression(right)?;

    match v {
        Object::Bool(a) => Ok(Object::Bool(!a)),
        v => Err(EvaluationError::PrefixOperationNotImplemented {
            operation: PrefixOperation::Negate,
            right: v,
        }),
    }
}

pub fn eval(operation: &PrefixOperation, right: &Expression) -> Result<Object, EvaluationError> {
    match operation {
        PrefixOperation::Negate => eval_negate(right),
        PrefixOperation::Negative => eval_negative(right),
    }
}
