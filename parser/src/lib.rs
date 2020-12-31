pub mod ast;
mod lexer;

use lexer::Token;

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equal,
    LessGreater,
    Sum,
    Product,
    Prefix,
}

fn infix_operator_precedence(operation: &ast::InfixOperation) -> Precedence {
    match operation {
        ast::InfixOperation::Sum => Precedence::Sum,
        ast::InfixOperation::Subtraction => Precedence::Sum,
        ast::InfixOperation::Product => Precedence::Product,
        ast::InfixOperation::Division => Precedence::Product,
        ast::InfixOperation::Equal => Precedence::Equal,
        ast::InfixOperation::NotEqual => Precedence::Equal,
        ast::InfixOperation::LessThan => Precedence::LessGreater,
        ast::InfixOperation::GreaterThan => Precedence::LessGreater,
        ast::InfixOperation::LessThanEqual => Precedence::LessGreater,
        ast::InfixOperation::GreaterThanEqual => Precedence::LessGreater,
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedToken { token: Token, expecting: String },
    FailedParsingInteger { string: String },
    MissingSemicolon,
    NonIdentifierExpression,
}

pub struct Parser<T: Iterator<Item = char>> {
    lexer: lexer::Lexer<T>,
    token_buffer: Vec<Token>,
}

impl<T: Iterator<Item = char>> Parser<T> {
    pub fn new(iter: T) -> Self {
        Parser {
            lexer: lexer::Lexer::new(iter),
            token_buffer: Vec::new(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.token_buffer.pop() {
            Some(t) => Some(t),
            None => self.lexer.next(),
        }
    }

    fn save_token(&mut self, t: Token) {
        self.token_buffer.push(t);
    }

    fn peek_next_token(&mut self) -> Option<&Token> {
        if self.token_buffer.is_empty() {
            match self.next_token() {
                Some(t) => self.save_token(t),
                None => return None,
            }
        }

        Some(&self.token_buffer[0])
    }

    fn skip_token(&mut self) -> Result<(), ParseError> {
        match self.next_token() {
            Some(_) => Ok(()),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn skip_token_expecting(&mut self, compare_to: Token) -> Result<(), ParseError> {
        if let Some(t) = self.next_token() {
            if t == compare_to {
                return Ok(());
            }

            return Err(ParseError::UnexpectedToken {
                token: t,
                expecting: format!("{:?}", compare_to),
            });
        }
        Err(ParseError::UnexpectedEnd)
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let identifier_token_option = self.next_token();
        if let Some(identifier_token) = identifier_token_option {
            if let Token::Identifier { name, .. } = identifier_token {
                Ok(name)
            } else {
                Err(ParseError::UnexpectedToken {
                    token: identifier_token,
                    expecting: String::from("Identifier"),
                })
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let identifier = self.parse_identifier()?;
        self.skip_token_expecting(Token::Assign)?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        Ok(ast::Statement::LetStatement {
            identifier,
            expression,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        Ok(ast::Statement::ReturnStatement { expression })
    }

    fn parse_integer_literal_expression(
        &mut self,
        value: String,
    ) -> Result<ast::Expression, ParseError> {
        match value.parse::<i64>() {
            Ok(i) => Ok(ast::Expression::IntegerLiteral { value: i }),
            Err(_) => Err(ParseError::FailedParsingInteger {
                string: String::from(value),
            }),
        }
    }

    fn parse_string_literal_expression(
        &mut self,
        value: String,
    ) -> Result<ast::Expression, ParseError> {
        Ok(ast::Expression::StringLiteral{value})
    }

    pub fn parse_statement_list(&mut self) -> Result<Vec<ast::Statement>, ParseError> {
        let mut block: Vec<ast::Statement> = Vec::new();

        loop {
            match self.peek_next_token() {
                None => break,
                Some(Token::CloseBrace) => break,
                _ => {}
            }

            let statement = self.parse_statement()?;
            match self.peek_next_token() {
                Some(Token::Semicolon) => {
                    self.skip_token()
                        .expect("We just peeked, so there must be a semicolon here");
                    block.push(statement);
                }
                _ => {
                    if let ast::Statement::ExpressionStatement { expression } = statement {
                        // If there is no semicolon after statement, and it is a expression,
                        // transform it to a return statement;
                        block.push(ast::Statement::ReturnStatement { expression });
                        break;
                    }
                    // No-semicolon is not allowed for other types of statements
                    return Err(ParseError::MissingSemicolon);
                }
            }
        }

        Ok(block)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<ast::Expression>, ParseError> {
        let mut list: Vec<ast::Expression> = Vec::new();
        self.skip_token_expecting(Token::OpenParenthesis)?;

        if let Some(Token::CloseParenthesis) = self.peek_next_token() {
            self.skip_token().expect("We just peeked");
            return Ok(list);
        };

        loop {
            let expression = self.parse_expression(Precedence::Lowest)?;

            list.push(expression);

            match self.peek_next_token() {
                Some(Token::Comma) => {
                    self.skip_token().expect("We just peeked");
                    continue;
                }
                Some(Token::CloseParenthesis) => {
                    self.skip_token().expect("We just peeked");
                    break;
                }
                Some(t) => {
                    return Err(ParseError::UnexpectedToken {
                        token: t.clone(),
                        expecting: String::from("comma, close parenthesis"),
                    })
                }
                None => return Err(ParseError::UnexpectedEnd),
            };
        }

        Ok(list)
    }

    fn parse_function_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let arguments_expressions = self.parse_expression_list()?;

        if !arguments_expressions
            .iter()
            .all(|ex| matches!(ex, ast::Expression::IdentifierExpression { .. }))
        {
            return Err(ParseError::NonIdentifierExpression);
        }

        let arguments = arguments_expressions
            .into_iter()
            .map(|ex| {
                if let ast::Expression::IdentifierExpression { identifier } = ex {
                    return identifier;
                }

                unreachable!();
            })
            .collect::<Vec<_>>();

        self.skip_token_expecting(Token::OpenBrace)?;
        let body: Vec<ast::Statement> = self.parse_statement_list()?;
        self.skip_token_expecting(Token::CloseBrace)?;

        Ok(ast::Expression::FunctionExpression { arguments, body })
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, ParseError> {
        self.skip_token_expecting(Token::OpenParenthesis)?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.skip_token_expecting(Token::CloseParenthesis)?;

        self.skip_token_expecting(Token::OpenBrace)?;
        let consequence: Vec<ast::Statement> = self.parse_statement_list()?;
        self.skip_token_expecting(Token::CloseBrace)?;

        match self.peek_next_token() {
            Some(Token::Else) => {
                self.skip_token()?;
                self.skip_token_expecting(Token::OpenBrace)?;
                let alternative: Vec<ast::Statement> = self.parse_statement_list()?;
                self.skip_token_expecting(Token::CloseBrace)?;

                Ok(ast::Expression::IfExpression {
                    condition: Box::new(condition),
                    consequence,
                    alternative: Some(alternative),
                })
            }

            _ => Ok(ast::Expression::IfExpression {
                condition: Box::new(condition),
                consequence,
                alternative: None,
            }),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        self.skip_token_expecting(Token::CloseParenthesis)?;

        Ok(expression)
    }

    fn parse_prefix_expression(
        &mut self,
        operation: ast::PrefixOperation,
    ) -> Result<ast::Expression, ParseError> {
        let expression = self.parse_expression(Precedence::Prefix)?;

        Ok(ast::Expression::PrefixExpression {
            operation,
            right: Box::new(expression),
        })
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, ParseError> {
        if let Some(token) = self.next_token() {
            return match token {
                Token::Integer { string } => self.parse_integer_literal_expression(string),
                Token::StringLiteral { string } => self.parse_string_literal_expression(string),
                Token::Identifier { name } => {
                    Ok(ast::Expression::IdentifierExpression { identifier: name })
                }
                Token::True => Ok(ast::Expression::Boolean { value: true }),
                Token::False => Ok(ast::Expression::Boolean { value: false }),
                Token::Bang => self.parse_prefix_expression(ast::PrefixOperation::Negate),
                Token::Minus => self.parse_prefix_expression(ast::PrefixOperation::Negative),
                Token::OpenParenthesis => self.parse_grouped_expression(),
                Token::If => self.parse_if_expression(),
                Token::Function => self.parse_function_expression(),
                t => Err(ParseError::UnexpectedToken {
                    token: t,
                    expecting: String::from("Prefix operator/Integer/Identifier"),
                }),
            };
        }

        Err(ParseError::UnexpectedEnd)
    }

    fn parse_call_expression(
        &mut self,
        function: ast::Expression,
    ) -> Result<ast::Expression, ParseError> {
        let arguments = self.parse_expression_list()?;
        Ok(ast::Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_infix_expression(
        &mut self,
        operation: ast::InfixOperation,
        left: ast::Expression,
    ) -> Result<ast::Expression, ParseError> {
        let precedence = infix_operator_precedence(&operation);
        match self.parse_expression(precedence) {
            Ok(exp) => Ok(ast::Expression::InfixExpression {
                operation,
                left: Box::new(left),
                right: Box::new(exp),
            }),
            Err(e) => Err(e),
        }
    }

    fn parse_infix(
        &mut self,
        left: ast::Expression,
        precedence: &Precedence,
    ) -> Option<Result<ast::Expression, ParseError>> {
        if let Some(token) = self.next_token() {
            let operation = match token {
                Token::Plus => ast::InfixOperation::Sum,
                Token::Asterisk => ast::InfixOperation::Product,
                Token::Slash => ast::InfixOperation::Division,
                Token::Minus => ast::InfixOperation::Subtraction,
                Token::Equal => ast::InfixOperation::Equal,
                Token::NotEqual => ast::InfixOperation::NotEqual,
                Token::LessThan => ast::InfixOperation::LessThan,
                Token::GreaterThan => ast::InfixOperation::GreaterThan,
                Token::LessThanEqual => ast::InfixOperation::LessThanEqual,
                Token::GreaterThanEqual => ast::InfixOperation::GreaterThanEqual,
                Token::OpenParenthesis => {
                    self.save_token(token);
                    return Some(self.parse_call_expression(left));
                }
                t => {
                    self.save_token(t);
                    return None;
                }
            };

            let new_precedence = infix_operator_precedence(&operation);

            if new_precedence <= *precedence {
                self.save_token(token);
                return None;
            }

            return Some(self.parse_infix_expression(operation, left));
        }

        None
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression, ParseError> {
        let mut left = self.parse_prefix()?;

        loop {
            let infix_opt = self.parse_infix(left.clone(), &precedence);

            if let Some(Ok(infix)) = infix_opt {
                left = infix;
            } else if let Some(Err(e)) = infix_opt {
                return Err(e);
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, ParseError> {
        if let Some(token) = self.next_token() {
            return match token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                t => {
                    // Try to parse expression as ExpressionStatement
                    self.save_token(t);
                    match self.parse_expression(Precedence::Lowest) {
                        Ok(expression) => Ok(ast::Statement::ExpressionStatement { expression }),
                        Err(e) => Err(e),
                    }
                }
            };
        }

        Err(ParseError::UnexpectedEnd)
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        self.parse_statement_list()
    }
}
