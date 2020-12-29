use parser::ast;
use parser::Parser;

fn parse(s: &str) -> ast::Program {
    let mut parser = Parser::new(s.chars());
    parser.parse_program().unwrap()
}

#[test]
fn test_let_statement() {
    let program = "let answer = 42;";

    let expected_ast = vec![ast::Statement::LetStatement {
        identifier: String::from("answer"),
        expression: ast::Expression::IntegerLiteral { value: 42 },
    }];

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_return_expression() {
    let program = "return 12;";

    let expected_ast = vec![ast::Statement::ReturnStatement {
        expression: ast::Expression::IntegerLiteral { value: 12 },
    }];

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_statement() {
    let program = "
        42;
        mandarina;
        true;
        false;
    ";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_prefix_expressions() {
    let program = "
        !n;
        -22;
    ";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_sum_expression() {
    let program = "42 +4;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Sum,
            left: Box::new(ast::Expression::IntegerLiteral { value: 42 }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 4 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_subtraction_expressions() {
    let program = "42-4;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Subtraction,
            left: Box::new(ast::Expression::IntegerLiteral { value: 42 }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 4 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_division_expressions() {
    let program = "mandarina / platan;";

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
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_product_expressions() {
    let program = "mandarina * 2;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Product,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_equal_expressions() {
    let program = "mandarina == 52;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::Equal,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 52 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_not_equal_expressions() {
    let program = "mandarina != 51;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::NotEqual,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_less_than_expressions() {
    let program = "mandarina < 51;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::LessThan,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_greater_than_expressions() {
    let program = "51 > mandarina;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::GreaterThan,
            left: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_less_than_equal_expressions() {
    let program = "51 <= mandarina;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::LessThanEqual,
            left: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
            right: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}
#[test]
fn test_greater_than_equal_expressions() {
    let program = "mandarina >= 51;";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::InfixExpression {
            operation: ast::InfixOperation::GreaterThanEqual,
            left: Box::new(ast::Expression::IdentifierExpression {
                identifier: String::from("mandarina"),
            }),
            right: Box::new(ast::Expression::IntegerLiteral { value: 51 }),
        },
    }];
    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_precedence_1() {
    let program = "5 + 2 * 10;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_precedence_1_b() {
    let program = "5 * 2 + 10;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_precedence_2() {
    let program = "-5 + 2;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_precedence_3() {
    let program = "a + b + c;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_expression_precedence_4() {
    let program = "a+b*c+d/e-f;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_grouped_expression_1() {
    let program = "(5 + 2) * 10;";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_grouped_expression_2() {
    let program = "5 * ( 2 + 10 );";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_grouped_expression_nested() {
    let program = "1 * ((4 + 5) * 8);";

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

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_if_expression() {
    let program = "
        if (x <= 7) {
            let z = x * 2;
            z
        };
        ";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::IfExpression {
            condition: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::LessThanEqual,
                left: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("x"),
                }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 7 }),
            }),
            consequence: vec![
                ast::Statement::LetStatement {
                    identifier: String::from("z"),
                    expression: ast::Expression::InfixExpression {
                        operation: ast::InfixOperation::Product,
                        left: Box::new(ast::Expression::IdentifierExpression {
                            identifier: String::from("x"),
                        }),
                        right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                    },
                },
                ast::Statement::ReturnStatement {
                    expression: ast::Expression::IdentifierExpression {
                        identifier: String::from("z"),
                    },
                },
            ],
            alternative: None,
        },
    }];

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_if_else_expression() {
    let program = "
        if (x <= 7) {
            let z = x * 2;
            z
        } else {
            14
        };
        ";

    let expected_ast = vec![ast::Statement::ExpressionStatement {
        expression: ast::Expression::IfExpression {
            condition: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::LessThanEqual,
                left: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("x"),
                }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 7 }),
            }),
            consequence: vec![
                ast::Statement::LetStatement {
                    identifier: String::from("z"),
                    expression: ast::Expression::InfixExpression {
                        operation: ast::InfixOperation::Product,
                        left: Box::new(ast::Expression::IdentifierExpression {
                            identifier: String::from("x"),
                        }),
                        right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                    },
                },
                ast::Statement::ReturnStatement {
                    expression: ast::Expression::IdentifierExpression {
                        identifier: String::from("z"),
                    },
                },
            ],
            alternative: Some(vec![ast::Statement::ReturnStatement {
                expression: ast::Expression::IntegerLiteral { value: 14 },
            }]),
        },
    }];

    assert_eq!(parse(program), expected_ast);
}

#[test]
fn test_assign_if_expression() {
    let program = "
        let a = if (x <= 7) {
            let z = x * 2;
            z
        };
        ";

    let expected_ast = vec![ast::Statement::LetStatement {
        identifier: String::from("a"),
        expression: ast::Expression::IfExpression {
            condition: Box::new(ast::Expression::InfixExpression {
                operation: ast::InfixOperation::LessThanEqual,
                left: Box::new(ast::Expression::IdentifierExpression {
                    identifier: String::from("x"),
                }),
                right: Box::new(ast::Expression::IntegerLiteral { value: 7 }),
            }),
            consequence: vec![
                ast::Statement::LetStatement {
                    identifier: String::from("z"),
                    expression: ast::Expression::InfixExpression {
                        operation: ast::InfixOperation::Product,
                        left: Box::new(ast::Expression::IdentifierExpression {
                            identifier: String::from("x"),
                        }),
                        right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                    },
                },
                ast::Statement::ReturnStatement {
                    expression: ast::Expression::IdentifierExpression {
                        identifier: String::from("z"),
                    },
                },
            ],
            alternative: None,
        },
    }];

    assert_eq!(parse(program), expected_ast);
}
