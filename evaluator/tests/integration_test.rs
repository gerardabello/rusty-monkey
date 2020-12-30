use parser::ast::InfixOperation;
use parser::Parser;

use evaluator::object::Object;
use evaluator::{eval_program, EvaluationError};

fn run(program: &str) -> Result<Object, EvaluationError> {
    let ast = Parser::new(program.chars()).parse_program().unwrap();
    eval_program(&ast)
}

#[test]
fn test_sum() {
    let program = "2 + 1";
    assert_eq!(run(program), Ok(Object::Integer(3)));
}

#[test]
fn test_sum_bool() {
    let program = "true + 1";
    assert_eq!(
        run(program),
        Err(EvaluationError::InfixOperationNotImplemented {
            operation: InfixOperation::Sum,
            left: Object::Bool(true),
            right: Object::Integer(1)
        })
    );
}

#[test]
fn test_product() {
    let program = "5*6";
    assert_eq!(run(program), Ok(Object::Integer(30)));
}

#[test]
fn test_subtraction() {
    let program = "66 - 11";
    assert_eq!(run(program), Ok(Object::Integer(55)));
}

#[test]
fn test_division() {
    let program = "100 / 4";
    assert_eq!(run(program), Ok(Object::Integer(25)));
}

#[test]
fn test_if_expression_true() {
    let program = "if (true) { 3 }";
    assert_eq!(run(program), Ok(Object::Integer(3)));
}

#[test]
fn test_if_expression_false() {
    let program = "if (false) { 3 }";
    assert_eq!(run(program), Ok(Object::Null));
}

#[test]
fn test_if_else_expression_true() {
    let program = "if (true) { 3 } else { 5 }";
    assert_eq!(run(program), Ok(Object::Integer(3)));
}

#[test]
fn test_if_else_expression_false() {
    let program = "if (false) { 3 } else { 5 }";
    assert_eq!(run(program), Ok(Object::Integer(5)));
}

#[test]
fn test_if_number() {
    let program = "if (1403) { 3 }";
    assert_eq!(
        run(program),
        Err(EvaluationError::UnexpectedType {
            value: Object::Integer(1403),
            expected: "bool"
        })
    );
}

#[test]
fn test_negative() {
    let program = "-145";
    assert_eq!(run(program), Ok(Object::Integer(-145)));
}

#[test]
fn test_negate() {
    let program = "!true";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}
