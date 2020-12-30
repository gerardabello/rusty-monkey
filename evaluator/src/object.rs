use parser::ast::{Statement};

#[derive(Clone, PartialEq, Debug)]
pub enum Object{
    Null,
    Integer(i64),
    Bool(bool),
    Function(Vec<String>,Vec<Statement>)
}
