use parser::Parser;

use evaluator::eval_program;
use evaluator::object::Object;


fn run (program: &str) -> Object {
    let ast = Parser::new(program.chars()).parse_program().unwrap();
    eval_program(&ast).unwrap()
}

#[test]
fn test_sum() {
    let program = "2 + 1";
    assert_eq!(run(program),Object::Integer(3));
}

#[test]
fn test_product() {
    let program = "5*6";
    assert_eq!(run(program),Object::Integer(30));
}

#[test]
fn test_subtraction() {
    let program = "66 - 11";
    assert_eq!(run(program),Object::Integer(55));
}

#[test]
fn test_division() {
    let program = "100 / 4";
    assert_eq!(run(program),Object::Integer(25));
}

#[test]
fn test_if_expression_true() {
    let program = "if (true) { 3 }";
    assert_eq!(run(program),Object::Integer(3));
}

#[test]
fn test_if_expression_false() {
    let program = "if (false) { 3 }";
    assert_eq!(run(program),Object::Null);
}

#[test]
fn test_if_else_expression_true() {
    let program = "if (true) { 3 } else { 5 }";
    assert_eq!(run(program),Object::Integer(3));
}

#[test]
fn test_if_else_expression_false() {
    let program = "if (false) { 3 } else { 5 }";
    assert_eq!(run(program),Object::Integer(5));
}
