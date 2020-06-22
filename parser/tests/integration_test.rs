use lexer::Token;
use parser::ast;
use parser::Parser;

fn parse_tokens(tokens: Vec<Token>) -> ast::Program {
    let mut parser = Parser::new(tokens.into_iter());
    parser.parse_program().unwrap()
}

#[test]
fn test_let_statement() {
    let tokens = vec![
        Token::Let,
        Token::Identifier {
            name: String::from("answer"),
        },
        Token::Assign,
        Token::Integer {
            string: String::from("42"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::LetStatement {
        identifier: String::from("answer"),
        expression: ast::Expression::IntegerLiteral { value: 42 },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_return_expression() {
    let tokens = vec![
        Token::Return,
        Token::Integer {
            string: String::from("12"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ReturnStatement {
        expression: ast::Expression::IntegerLiteral { value: 12 },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_statement() {
    let tokens = vec![
        Token::Integer {
            string: String::from("42"),
        },
        Token::Semicolon,
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::IntegerLiteral { value: 42 },
        },
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            },
        },
    ];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_negative_expression() {
    let tokens = vec![
        Token::Minus,
        Token::Integer {
            string: String::from("22"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::PrefixExpression {
            operation: ast::PrefixOperation::Negative,
            right: Box::new(ast::Expression::IntegerLiteral { value: 22 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_negate_expression() {
    let tokens = vec![
        Token::Bang,
        Token::Identifier {
            name: String::from("n"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::PrefixExpression {
            operation: ast::PrefixOperation::Negate,
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("n"),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_plus_expression() {
    let tokens = vec![
        Token::Integer {
            string: String::from("42"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("4"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::IntegerLiteral { value: 42 }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 4 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_1() {
    let tokens = vec![
        Token::Integer {
            string: String::from("5"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("5"),
        },
        Token::Asterisk,
        Token::Integer {
            string: String::from("10"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
            right: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Product,
                left: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_2() {
    let tokens = vec![
        Token::Integer {
            string: String::from("5"),
        },
        Token::Asterisk,
        Token::Integer {
            string: String::from("5"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("10"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Product,
                left: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}
