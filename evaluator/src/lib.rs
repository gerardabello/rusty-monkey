pub mod object;

use parser::ast::{Expression, InfixOperation, Statement};

use object::Object;

#[derive(PartialEq, Debug)]
pub enum EvaluationError {
    OperationNotImplemented{operation: InfixOperation, left: Expression, right :Expression},
    UnexpectedType{value: Expression, expected: &'static str}
}

fn eval_sum_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a + b)),
        _ => Err(EvaluationError::OperationNotImplemented{operation: InfixOperation::Sum, left: left.clone(), right: right.clone()})
    }
}

fn eval_product_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a * b)),
        _ => Err(EvaluationError::OperationNotImplemented{operation: InfixOperation::Product, left: left.clone(), right: right.clone()})
    }
}

fn eval_subtract_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a - b)),
        _ => Err(EvaluationError::OperationNotImplemented{operation: InfixOperation::Subtraction, left: left.clone(), right: right.clone()})
    }
}

fn eval_division_expression(left: &Expression, right: &Expression) -> Result<Object, EvaluationError> {
    let left_v = eval_expression(left)?;
    let right_v = eval_expression(right)?;

    match (left_v, right_v) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Object::Integer(a / b)),
        _ => Err(EvaluationError::OperationNotImplemented{operation: InfixOperation::Division, left: left.clone(), right: right.clone()})
    }
}

fn eval_infix_expression(
    operation: &InfixOperation,
    left: &Expression,
    right: &Expression,
) -> Result<Object, EvaluationError> {
    match operation {
        InfixOperation::Sum => eval_sum_expression(left, right),
        InfixOperation::Product => eval_product_expression(left, right),
        InfixOperation::Subtraction => eval_subtract_expression(left, right),
        InfixOperation::Division => eval_division_expression(left, right),
        _ => Err(EvaluationError::OperationNotImplemented{operation: operation.clone(), left: left.clone(), right: right.clone()})
    }
}

fn eval_if_expression(
    condition: &Expression,
    consequence: &[Statement],
    alternative: &Option<Vec<Statement>>,
) -> Result<Object, EvaluationError> {
    match eval_expression(condition)? {
        Object::Bool(true) => eval_statements(consequence),
        Object::Bool(false) => match alternative {
            None => Ok(Object::Null),
            Some(alt) => eval_statements(alt),
        },
        _ => Err(EvaluationError::UnexpectedType{value: condition.clone(), expected: "bool"})
    }
}

fn eval_expression(expression: &Expression) -> Result<Object, EvaluationError> {
    match expression {
        Expression::IntegerLiteral { value } => Ok(Object::Integer(*value)),
        Expression::Boolean { value } => Ok(Object::Bool(*value)),
        Expression::InfixExpression {
            operation,
            right,
            left,
        } => eval_infix_expression(operation, left, right),
        Expression::IfExpression { condition, consequence, alternative } => eval_if_expression(condition, consequence, alternative),
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
