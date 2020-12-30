#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
}
