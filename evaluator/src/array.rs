use std::cell::RefCell;
use std::rc::Rc;

use parser::ast::Expression;

use super::{env::Environment, eval_expression, object::Object, EvaluationError};

pub fn eval_indexing(
    arr: Vec<Object>,
    index: Object,
) -> Result<Object, EvaluationError> {

    match index {
        Object::Integer(i) => {
            if arr.len() <= i as usize {
                return Err(EvaluationError::IndexOutOfBounds {
                    value: Object::Array(arr),
                    index: i as usize,
                });
            }
            Ok(arr[i as usize].clone())
        }
        i => Err(EvaluationError::NotIndexable { value: Object::Array(arr), index: Some(i) }),
    }
}

pub fn eval_array(
    env: &Rc<RefCell<Environment>>,
    array: &[Expression],
) -> Result<Object, EvaluationError> {
    let values: Result<Vec<Object>, EvaluationError> =
        array.iter().map(|ex| eval_expression(env, ex)).collect();

    match values {
        Ok(v) => Ok(Object::Array(v)),
        Err(e) => Err(e),
    }
}
