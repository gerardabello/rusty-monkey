use std::cell::RefCell;
use std::rc::Rc;

use parser::ast::{Expression, PrefixOperation};

use super::{env::Environment, eval_expression, object::Object, EvaluationError};

fn eval_negative(
    env: &Rc<RefCell<Environment>>,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let v = eval_expression(env, right)?;

    match v {
        Object::Integer(a) => Ok(Object::Integer(-a)),
        v => Err(EvaluationError::PrefixOperationNotImplemented {
            operation: PrefixOperation::Negative,
            right: v,
        }),
    }
}

fn eval_negate(
    env: &Rc<RefCell<Environment>>,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    let v = eval_expression(env, right)?;

    match v {
        Object::Bool(a) => Ok(Object::Bool(!a)),
        v => Err(EvaluationError::PrefixOperationNotImplemented {
            operation: PrefixOperation::Negate,
            right: v,
        }),
    }
}

pub fn eval(
    env: &Rc<RefCell<Environment>>,
    operation: &PrefixOperation,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    match operation {
        PrefixOperation::Negate => eval_negate(env, right),
        PrefixOperation::Negative => eval_negative(env, right),
    }
}
