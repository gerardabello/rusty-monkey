use lexer::Token;

pub mod ast;

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equal,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
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
}

pub struct Parser<T: Iterator<Item = Token>> {
    iter: T,
    token_buffer: Vec<Token>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(iter: T) -> Self {
        Parser {
            iter,
            token_buffer: Vec::new(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.token_buffer.pop() {
            Some(t) => Some(t),
            None => self.iter.next(),
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
        value: &str,
    ) -> Result<ast::Expression, ParseError> {
        match value.parse::<i64>() {
            Ok(i) => Ok(ast::Expression::IntegerLiteral { value: i }),
            Err(_) => Err(ParseError::FailedParsingInteger {
                string: String::from(value),
            }),
        }
    }

    pub fn parse_statement_block(&mut self) -> Result<Vec<ast::Statement>, ParseError> {
        let mut block: Vec<ast::Statement> = Vec::new();

        self.skip_token_expecting(Token::OpenBrace)?;

        loop {
            let statement = self.parse_statement()?;
            match self.peek_next_token() {
                Some(Token::Semicolon) => {
                    self.skip_token().expect("We just peeked, so there must be a semicolon here");
                    block.push(statement);
                }
                Some(_) => {
                    if let ast::Statement::ExpressionStatement { expression } = statement {
                        // If there is no semicolon after statement, and it is a expression,
                        // transform it to a return statement;
                        block.push(ast::Statement::ReturnStatement { expression });
                        break;
                    }
                    // No-semicolon is not allowed for other types of statements
                    return Err(ParseError::MissingSemicolon);
                }
                None => return Err(ParseError::UnexpectedEnd),
            }
        }

        self.skip_token_expecting(Token::CloseBrace)?;

        Ok(block)
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, ParseError> {
        self.skip_token_expecting(Token::OpenParenthesis)?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.skip_token_expecting(Token::CloseParenthesis)?;

        let consequence: Vec<ast::Statement> = self.parse_statement_block()?;

        match self.peek_next_token() {
            Some(Token::Else) => {
                self.skip_token()?;
                let alternative: Vec<ast::Statement> = self.parse_statement_block()?;

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
                Token::Integer { string } => self.parse_integer_literal_expression(&string),
                Token::Identifier { name } => {
                    Ok(ast::Expression::IdentifierExpression { identifier: name })
                }
                Token::True => Ok(ast::Expression::Boolean { value: true }),
                Token::False => Ok(ast::Expression::Boolean { value: false }),
                Token::Bang => self.parse_prefix_expression(ast::PrefixOperation::Negate),
                Token::Minus => self.parse_prefix_expression(ast::PrefixOperation::Negative),
                Token::OpenParenthesis => self.parse_grouped_expression(),
                Token::If => self.parse_if_expression(),
                t => Err(ParseError::UnexpectedToken {
                    token: t,
                    expecting: String::from("Prefix operator/Integer/Identifier"),
                }),
            };
        }

        Err(ParseError::UnexpectedEnd)
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

        Some(Err(ParseError::UnexpectedEnd))
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
        let mut program: ast::Program = Vec::new();
        loop {
            if self.peek_next_token().is_none() {
                break;
            }
            let statement = self.parse_statement()?;
            self.skip_token_expecting(Token::Semicolon)?;
            program.push(statement);
        }

        Ok(program)
    }
}
