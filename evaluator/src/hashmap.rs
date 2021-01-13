use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use parser::ast::Expression;

use super::{env::Environment, eval_expression, object::Object, EvaluationError};

pub fn check_key_type(key: &Object) -> Result<(), EvaluationError> {
    match key {
        Object::Null => Ok(()),
        Object::Bool(_) => Ok(()),
        Object::Integer(_) => Ok(()),
        Object::Str(_) => Ok(()),
        _ =>  Err(EvaluationError::NotHashable { value: key.clone() }),
    }
}

pub fn eval_indexing(
    hm: HashMap<Object, Object>,
    index: Object,
) -> Result<Object, EvaluationError> {
    check_key_type(&index)?;
    match hm.get(&index) {
        Some(v) => Ok(v.clone()),
        None => Ok(Object::Null),
    }
}

pub fn eval_hashmap(
    env: &Rc<RefCell<Environment>>,
    pairs: &[(Expression, Expression)],
) -> Result<Object, EvaluationError> {
    let mut hm = HashMap::new();

    for (key_ex, val_ex) in pairs {
        match (eval_expression(env, key_ex), eval_expression(env, val_ex)) {
            (Ok(key), Ok(val)) => {
                check_key_type(&key)?;

                hm.insert(key, val);
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };
    }

    Ok(Object::HashMap(hm))
}
