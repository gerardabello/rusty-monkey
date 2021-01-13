use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use parser::ast::Statement;

use super::env::Environment;

#[derive(Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
    Str(String),
    Array(Vec<Object>),
    HashMap(HashMap<Object, Object>),
    Function(Vec<String>, Vec<Statement>, Rc<RefCell<Environment>>),
    BuiltInFunction(String),
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Null => "null".hash(state),
            Object::Integer(v) => v.hash(state),
            Object::Bool(v) => v.hash(state),
            Object::Str(v) => v.hash(state),

            Object::Array(_) => unreachable!("Should not atempt to calculate hash of array"),
            Object::HashMap(_) => unreachable!("Should not atempt to calculate hash of hashmap"),
            Object::Function(_, _, _) => {
                unreachable!("Should not atempt to calculate hash of functioj")
            }
            Object::BuiltInFunction(_) => {
                unreachable!("Should not atempt to calculate hash of builtin")
            }
        }
    }
}

fn keys_match<T: Eq + Hash, U: Eq>(map1: &HashMap<T, U>, map2: &HashMap<T, U>) -> bool {
    map1.len() == map2.len()
        && map1.keys().all(|k| map2.contains_key(k))
        && map1.keys().all(|k| map1[k] == map2[k])
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(v1), Object::Integer(v2)) => v1 == v2,
            (Object::Bool(v1), Object::Bool(v2)) => v1 == v2,
            (Object::Str(v1), Object::Str(v2)) => v1 == v2,
            (Object::HashMap(v1), Object::HashMap(v2)) => keys_match(v1, v2),
            (Object::Array(v1), Object::Array(v2)) => {
                if v1.len() != v2.len() {
                    return false;
                }

                v1.iter().zip(v2.iter()).all(|(e1, e2)| e1 == e2)
            }
            (Object::BuiltInFunction(v1), Object::BuiltInFunction(v2)) => v1 == v2,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}

impl Eq for Object {}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.debug_tuple("Null").finish(),
            Object::Integer(v) => f.debug_tuple("Integer").field(v).finish(),
            Object::Bool(v) => f.debug_tuple("Bool").field(v).finish(),
            Object::Str(v) => f.debug_tuple("Str").field(v).finish(),
            Object::Array(v) => f.debug_list().entries(v.iter()).finish(),
            Object::HashMap(v) => f.debug_map().entries(v.iter()).finish(),
            Object::Function(args, _, _) => f.debug_tuple("Function").field(args).finish(),
            Object::BuiltInFunction(name) => f.debug_tuple("BuiltInFunction").field(name).finish(),
        }
    }
}

fn display_array<T: std::fmt::Display>(
    array: &[T],
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "[")?;
    for (index, item) in array.iter().enumerate() {
        write!(f, "{}", item)?;
        if index < array.len() - 1 {
            write!(f, ", ")?;
        }
    }
    write!(f, "]")
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "NULL"),
            Object::Integer(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Str(v) => write!(f, "{}", v),
            Object::HashMap(_) => write!(f, "HashMap"), // TODO
            Object::Array(v) => display_array(v, f),
            Object::Function(args, _, _) => {
                write!(f, "]")?;
                display_array(args, f)
            }
            Object::BuiltInFunction(name) => write!(f, "builtin({})", name),
        }
    }
}
