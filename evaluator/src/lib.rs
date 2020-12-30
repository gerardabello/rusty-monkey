pub mod object;

use parser::ast::{Expression, InfixOperation, Statement};

use object::Object;

#[derive(PartialEq, Debug)]
pub enum EvaluationError {}

fn eval_sum_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    let ret = match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Object::Integer(a + b),
        _ => panic!("Sum not implemented for {:?} and {:?}", left, right),
    };

    Ok(ret)
}

fn eval_product_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    let ret = match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Object::Integer(a * b),
        _ => panic!("Product not implemented for {:?} and {:?}", left, right),
    };

    Ok(ret)
}

fn eval_infix_expression(
    operation: &InfixOperation,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    match operation {
        InfixOperation::Sum => eval_sum_expression(left, right),
        InfixOperation::Product => eval_product_expression(left, right),
        _ => panic!("Infix operation {:?} not implemented", operation),
    }
}

fn eval_expression(expression: &Expression) -> Result<Object, EvaluationError> {
    match expression {
        Expression::IntegerLiteral { value } => Ok(Object::Integer(*value)),
        Expression::InfixExpression {
            operation,
            right,
            left,
        } => eval_infix_expression(operation, left, right),
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
