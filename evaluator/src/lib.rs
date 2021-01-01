pub mod object;

mod condition;
mod env;
mod function;
mod infix;
mod prefix;
mod builtin;

use std::cell::RefCell;
use std::rc::Rc;

use env::Environment;
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
    NotCallable {
        value: Object,
    },
}

fn eval_expression(
    env: &Rc<RefCell<Environment>>,
    expression: &Expression,
) -> Result<Object, EvaluationError> {
    match expression {
        Expression::IntegerLiteral { value } => Ok(Object::Integer(*value)),
        Expression::StringLiteral { value } => Ok(Object::Str(value.clone())),
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

fn eval_statements(
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

pub fn eval_program(program: &[Statement]) -> Result<Object, EvaluationError> {
    let env = Rc::new(RefCell::new(Environment::new()));
    builtin::set_builtins_to_env(&env);
    eval_statements(&env, program)
}
