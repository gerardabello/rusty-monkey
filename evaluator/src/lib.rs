pub mod object;

mod array;
mod hashmap;
mod builtin;
mod condition;
mod env;
mod function;
mod infix;
mod prefix;

use std::cell::RefCell;
use std::rc::Rc;

pub use env::Environment;
pub use builtin::set_builtins_to_env;
use object::Object;
use parser::ast::{Expression, InfixOperation, PrefixOperation, Statement};

#[derive(PartialEq, Debug)]
pub enum EvaluationError {
    InfixOperationNotImplemented {
        operation: InfixOperation,
        left: Object,
        right: Object,
    },
    PrefixOperationNotImplemented {
        operation: PrefixOperation,
        right: Object,
    },
    InvalidArguments {
        values: Vec<Object>,
        expected: &'static str,
    },
    UnexpectedType {
        value: Object,
        expected: &'static str,
    },
    IndexOutOfBounds {
        value: Object,
        index: usize,
    },
    NotHashable {
        value: Object,
    },
    NotCallable {
        value: Object,
    },
    NotIndexable{
        value: Object,
        index: Option<Object>,
    },
}

fn eval_expression(
    env: &Rc<RefCell<Environment>>,
    expression: &Expression,
) -> Result<Object, EvaluationError> {
    match expression {
        Expression::IntegerLiteral { value } => Ok(Object::Integer(*value)),
        Expression::StringLiteral { value } => Ok(Object::Str(value.clone())),
        Expression::Array { array } => array::eval_array(env, array),
        Expression::HashMap { pairs } => hashmap::eval_hashmap(env, pairs),
        Expression::Index { array, index } =>  {
    let array_v = eval_expression(env, array)?;
    let index_v = eval_expression(env, index)?;

    match array_v {
        Object::Array(arr) => array::eval_indexing(arr, index_v),
        Object::HashMap(hm) => hashmap::eval_indexing(hm, index_v),
        v => Err(EvaluationError::NotIndexable { value: v, index: None }),
    }
        },
        Expression::Boolean { value } => Ok(Object::Bool(*value)),
        Expression::IdentifierExpression { identifier } => Ok(Environment::get_rr(env, identifier)),
        Expression::InfixExpression {
            operation,
            right,
            left,
        } => infix::eval(env, operation, left, right),
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => condition::eval(env, condition, consequence, alternative),
        Expression::PrefixExpression { operation, right } => prefix::eval(env, operation, right),
        Expression::FunctionExpression { arguments, body } => {
            function::eval_function(env, arguments, body)
        }
        Expression::CallExpression {
            arguments,
            function,
        } => function::eval_call(env, function, arguments),
    }
}

fn eval_statement(
    env: &Rc<RefCell<Environment>>,
    statement: &Statement,
) -> Result<Option<Object>, EvaluationError> {
    match statement {
        Statement::ReturnStatement { expression } => match eval_expression(env, expression) {
            Err(e) => Err(e),
            Ok(v) => Ok(Some(v)),
        },
        Statement::LetStatement {
            identifier,
            expression,
        } => {
            let val = eval_expression(env, expression)?;
            Environment::set_rr(env, identifier.clone(), val);
            Ok(None)
        }
        Statement::ExpressionStatement { expression } => {
            eval_expression(env, expression)?;
            Ok(None)
        }
    }
}

pub fn eval_statements(
    env: &Rc<RefCell<Environment>>,
    statements: &[Statement],
) -> Result<Object, EvaluationError> {
    for statement in statements {
        match eval_statement(env, statement) {
            Err(e) => return Err(e),
            Ok(Some(v)) => return Ok(v),
            Ok(None) => {}
        }
    }

    Ok(Object::Null)
}

pub fn new_environment() ->Rc<RefCell<Environment>> {
    let env = Rc::new(RefCell::new(Environment::new()));
    set_builtins_to_env(&env);
    env
}

pub fn eval_program(program: &[Statement]) -> Result<Object, EvaluationError> {
    let env = new_environment();
    eval_statements(&env, program)
}
