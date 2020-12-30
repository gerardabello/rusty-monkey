pub mod object;

mod infix;
mod prefix;
mod condition;

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
    UnexpectedType {
        value: Object,
        expected: &'static str,
    },
}


fn eval_expression(expression: &Expression) -> Result<Object, EvaluationError> {
    match expression {
        Expression::IntegerLiteral { value } => Ok(Object::Integer(*value)),
        Expression::Boolean { value } => Ok(Object::Bool(*value)),
        Expression::InfixExpression {
            operation,
            right,
            left,
        } => infix::eval(operation, left, right),
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => condition::eval(condition, consequence, alternative),
        Expression::PrefixExpression { operation, right } => prefix::eval(operation, right),
        _ => panic!("Not implemented"),
    }
}

fn eval_statement(statement: &Statement) -> Result<Option<Object>, EvaluationError> {
    match statement {
        Statement::ReturnStatement { expression } => match eval_expression(expression) {
            Err(e) => Err(e),
            Ok(v) => Ok(Some(v)),
        },
        _ => panic!("Not implemented"),
    }
}

fn eval_statements(statements: &[Statement]) -> Result<Object, EvaluationError> {
    for statement in statements {
        match eval_statement(statement) {
            Err(e) => return Err(e),
            Ok(Some(v)) => return Ok(v),
            Ok(None) => {}
        }
    }

    Ok(Object::Null)
}

pub fn eval_program(program: &[Statement]) -> Result<Object, EvaluationError> {
    eval_statements(program)
}
