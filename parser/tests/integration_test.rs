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
        Token::True,
        Token::Semicolon,
        Token::False,
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
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::Boolean { value: true },
        },
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::Boolean { value: false },
        },
    ];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_prefix_expressions() {
    let tokens = vec![
        Token::Bang,
        Token::Identifier {
            name: String::from("n"),
        },
        Token::Semicolon,
        Token::Minus,
        Token::Integer {
            string: String::from("22"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::PrefixExpression {
                operation: ast::PrefixOperation::Negate,
                right: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("n"),
                }),
            },
        },
        ast::Statement::ExpressionStatement {
            expression: ast::Expression::PrefixExpression {
                operation: ast::PrefixOperation::Negative,
                right: Box::new(ast::Expression::IntegerLiteral { value: 22 }),
            },
        },
    ];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_sum_expression() {
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
fn test_subtraction_expressions() {
    let tokens = vec![
        Token::Integer {
            string: String::from("42"),
        },
        Token::Minus,
        Token::Integer {
            string: String::from("4"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Subtraction,
            left: Box::new(ast::Expression::IntegerLiteral { value: 42 }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 4 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_division_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::Slash,
        Token::Identifier {
            name: String::from("platan"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Division,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("platan"),
            }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_product_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::Asterisk,
        Token::Identifier {
            name: String::from("platan"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Product,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("platan"),
            }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_equal_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::Equal,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Equal,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_not_equal_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::NotEqual,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::NotEqual,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_less_than_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::LessThan,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::LessThan,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_greater_than_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::GreaterThan,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::GreaterThan,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_less_than_equal_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::LessThanEqual,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::LessThanEqual,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse_tokens(tokens), expected_ast);
}
#[test]
fn test_greater_than_equal_expressions() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("mandarina"),
        },
        Token::GreaterThanEqual,
        Token::Integer {
            string: String::from("51"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::GreaterThanEqual,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
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
            string: String::from("2"),
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
                left: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_1_b() {
    let tokens = vec![
        Token::Integer {
            string: String::from("5"),
        },
        Token::Asterisk,
        Token::Integer {
            string: String::from("2"),
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
                right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_2() {
    let tokens = vec![
        Token::Minus,
        Token::Integer {
            string: String::from("5"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("2"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::PrefixExpression {
                operation: ast::PrefixOperation::Negative,
                right: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_3() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("a"),
        },
        Token::Plus,
        Token::Identifier {
            name: String::from("b"),
        },
        Token::Plus,
        Token::Identifier {
            name: String::from("c"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Sum,
                left: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("a"),
                }),
                right: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("b"),
                }),
            }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("c"),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_expression_precedence_4() {
    let tokens = vec![
        Token::Identifier {
            name: String::from("a"),
        },
        Token::Plus,
        Token::Identifier {
            name: String::from("b"),
        },
        Token::Asterisk,
        Token::Identifier {
            name: String::from("c"),
        },
        Token::Plus,
        Token::Identifier {
            name: String::from("d"),
        },
        Token::Slash,
        Token::Identifier {
            name: String::from("e"),
        },
        Token::Minus,
        Token::Identifier {
            name: String::from("f"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Subtraction,
            left: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Sum,
                left: Box::new(ast::Expression::InfixExpression {
                    operation: ast::InfixOperation::Sum,
                    left: Box::new(ast::Expression::IdentifierExpression {
                        identifier: String::from("a"),
                    }),
                    right: Box::new(ast::Expression::InfixExpression {
                        operation: ast::InfixOperation::Product,
                        left: Box::new(ast::Expression::IdentifierExpression {
                            identifier: String::from("b"),
                        }),

                        right: Box::new(ast::Expression::IdentifierExpression {
                            identifier: String::from("c"),
                        }),
                    }),
                }),
                right: Box::new(ast::Expression::InfixExpression {
                    operation: ast::InfixOperation::Division,
                    left: Box::new(ast::Expression::IdentifierExpression {
                        identifier: String::from("d"),
                    }),

                    right: Box::new(ast::Expression::IdentifierExpression {
                        identifier: String::from("e"),
                    }),
                }),
            }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("f"),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_grouped_expression_1() {
    let tokens = vec![
        Token::OpenParenthesis,
        Token::Integer {
            string: String::from("5"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("2"),
        },
        Token::CloseParenthesis,
        Token::Asterisk,
        Token::Integer {
            string: String::from("10"),
        },
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Product,
            left: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Sum,
                left: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_grouped_expression_2() {
    let tokens = vec![
        Token::Integer {
            string: String::from("5"),
        },
        Token::Asterisk,
        Token::OpenParenthesis,
        Token::Integer {
            string: String::from("2"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("10"),
        },
        Token::CloseParenthesis,
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Product,
            left: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
            right: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Sum,
                left: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 10 }),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}

#[test]
fn test_grouped_expression_nested() {
    let tokens = vec![
        Token::Integer {
            string: String::from("1"),
        },
        Token::Asterisk,
        Token::OpenParenthesis,
        Token::OpenParenthesis,
        Token::Integer {
            string: String::from("4"),
        },
        Token::Plus,
        Token::Integer {
            string: String::from("5"),
        },
        Token::CloseParenthesis,
        Token::Asterisk,
        Token::Integer {
            string: String::from("8"),
        },
        Token::CloseParenthesis,
        Token::Semicolon,
    ];

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Product,
            left: Box::new(ast::Expression::IntegerLiteral { value: 1 }),
            right: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::Product,
                left: Box::new(ast::Expression::InfixExpression {
                    operation: ast::InfixOperation::Sum,
                    left: Box::new(ast::Expression::IntegerLiteral { value: 4 }),
                    right: Box::new(ast::Expression::IntegerLiteral { value: 5 }),
                }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 8 }),
            }),
        },
    }];

    assert_eq!(parse_tokens(tokens), expected_ast);
}
