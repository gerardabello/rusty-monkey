use std::cell::RefCell;
use std::rc::Rc;

use parser::ast::Statement;

use super::env::Environment;

#[derive(Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
    Str(String),
    Function(Vec<String>, Vec<Statement>, Rc<RefCell<Environment>>),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(v1), Object::Integer(v2)) => v1 == v2,
            (Object::Bool(v1), Object::Bool(v2)) => v1 == v2,
            (Object::Str(v1), Object::Str(v2)) => v1 == v2,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.debug_tuple("Null").finish(),
            Object::Integer(v) => f.debug_tuple("Integer").field(v).finish(),
            Object::Bool(v) => f.debug_tuple("Bool").field(v).finish(),
            Object::Str(v) => f.debug_tuple("Str").field(v).finish(),
            Object::Function(args, _, _) => f.debug_tuple("Function").field(args).finish(),
        }
    }
}
