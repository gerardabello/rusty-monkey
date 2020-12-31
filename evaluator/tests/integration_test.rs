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
fn test_string_concat() {
    let program = "\"hello\" + \" \" + \"world\"";
    assert_eq!(run(program), Ok(Object::Str(String::from("hello world"))));
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
    let program = "if (false) { 3 } else { \"cool\" }";
    assert_eq!(run(program), Ok(Object::Str(String::from("cool"))));
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

#[test]
fn test_less_than_true() {
    let program = "1 < 2";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_less_than_equal() {
    let program = "1 < 1";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_less_than_false() {
    let program = "6 < 1";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_greater_than_true() {
    let program = "66 > 65";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_greater_than_equal() {
    let program = "7 > 7";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_greater_than_false() {
    let program = "1002 > 2221";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_less_than_equal_true() {
    let program = "1 <= 2";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_less_than_equal_equal() {
    let program = "1 <= 1";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_less_than_equal_false() {
    let program = "6 <= 1";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_greater_than_equal_true() {
    let program = "66 >= 65";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_greater_than_equal_equal() {
    let program = "7 >= 7";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}

#[test]
fn test_greater_than_equal_false() {
    let program = "1002 >= 2221";
    assert_eq!(run(program), Ok(Object::Bool(false)));
}

#[test]
fn test_multiple_return() {
    let program = "return 10; return 6;";
    assert_eq!(run(program), Ok(Object::Integer(10)));
}

#[test]
fn test_let() {
    let program = "let x = 10 * 5; x";
    assert_eq!(run(program), Ok(Object::Integer(50)));
}

#[test]
fn test_double_let() {
    let program = "let x = 7; let y = x * 2; y";
    assert_eq!(run(program), Ok(Object::Integer(14)));
}

#[test]
fn test_function_1() {
    let program = "
    let add= fn (a,b) { a + b };
    add(5,6)
        ";
    assert_eq!(run(program), Ok(Object::Integer(11)));
}

#[test]
fn test_call_null() {
    let program = "
    add(5)
        ";
    assert_eq!(
        run(program),
        Err(EvaluationError::NotCallable {
            value: Object::Null
        })
    );
}

#[test]
fn test_call_integer() {
    let program = "
    3(5)
        ";
    assert_eq!(
        run(program),
        Err(EvaluationError::NotCallable {
            value: Object::Integer(3)
        })
    );
}

#[test]
fn test_function_2() {
    let program = "
    let addDoubles = fn (a,b) {
        let ad = a * 2;
        let bd = b * 2;
        return ad + bd;
    };

    addDoubles(5,6)
        ";
    assert_eq!(run(program), Ok(Object::Integer(22)));
}

#[test]
fn test_recursion() {
    let program = "
    let factorial = fn (n) {
      if (n == 0) {
        1
      } else{
        n * factorial(n-1)
      }
    };

    factorial(8)
        ";
    assert_eq!(run(program), Ok(Object::Integer(40_320)));
}

#[test]
fn test_hof() {
    let program = "
    let twoTimes = fn (f, n) {
        f(f(n))
    };

    let double = fn(n) { 2 * n };

    twoTimes(double, 4)
        ";
    assert_eq!(run(program), Ok(Object::Integer(16)));
}

#[test]
fn test_partial_application() {
    let program = "
    let twoTimes = fn (f) {
        fn (n) {
          f(f(n))
        }
    };

    let double = fn(n) { 2 * n };

    let byFour = twoTimes(double);

    byFour(3)
        ";
    assert_eq!(run(program), Ok(Object::Integer(12)));
}

#[test]
fn test_scope_1() {
    let program = "
    let c = 3;
    let returnC = fn () {
      c
    };

    returnC()
        ";
    assert_eq!(run(program), Ok(Object::Integer(3)));
}

#[test]
fn test_scope_2() {
    let program = "
    let returnC = fn () {
      c
    };

    let c = 3;

    returnC()
        ";
    assert_eq!(run(program), Ok(Object::Integer(3)));
}

#[test]
fn test_scope_3() {
    let program = "
    let returnC = fn () {
      c
    };

    fn () {
        let c = 3;
        returnC()
    }()
        ";
    assert_eq!(run(program), Ok(Object::Null));
}

#[test]
fn test_recursion_100() {
    let program = "
    let counter = fn (x) {
      if (x>100) {
        return true;
      } else {
        let foobar = 9999;
        counter(x+1)
      }
    };

    counter(0)
        ";
    assert_eq!(run(program), Ok(Object::Bool(true)));
}
