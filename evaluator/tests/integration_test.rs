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
