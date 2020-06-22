use lexer::Token;

pub mod ast;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedToken { token: Token, expecting: String },
    FailedParsingInteger { string: String },
}

pub struct Parser<T: Iterator<Item = Token>> {
    iter: T,
    saved_token: Option<Token>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(iter: T) -> Self {
        Parser {
            iter,
            saved_token: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let saved_token = std::mem::replace(&mut self.saved_token, None);
        if let Some(t) = saved_token {
            self.saved_token = None;
            return Some(t);
        }
        if let Some(t) = self.iter.next() {
            return Some(t);
        }
        None
    }

    fn save_token(&mut self, t: Token) {
        self.saved_token = Some(t)
    }

    fn skip_token_expecting(&mut self, compare_to: Token) -> Result<(), ParseError> {
        if let Some(t) = self.next_token() {
            if t == compare_to {
                return Ok(());
            }
            return Err(ParseError::UnexpectedToken { token: t, expecting: format!("{:?}", compare_to) });
        }
        Err(ParseError::UnexpectedEnd)
    }

    fn parse_identifier(&mut self) -> Result<ast::Identifier, ParseError> {
        let identifier_token_option = self.next_token();
        if let Some(identifier_token) = identifier_token_option {
            if let Token::Identifier { name, .. } = identifier_token {
                Ok(ast::Identifier { name })
            } else {
                Err(ParseError::UnexpectedToken {
                    token: identifier_token, expecting: String::from("Identifier")
                })
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let identifier = self.parse_identifier()?;
        self.skip_token_expecting(Token::Assign)?;
        let expression = self.parse_expression()?;
        Ok(ast::Statement::LetStatement {
            identifier,
            expression,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression = self.parse_expression()?;
        Ok(ast::Statement::ReturnStatement {
            expression,
        })
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

    fn parse_expression(&mut self) -> Result<ast::Expression, ParseError> {
        if let Some(token) = self.next_token() {
            return match token {
                Token::Integer { string } => self.parse_integer_literal_expression(&string),
                t => Err(ParseError::UnexpectedToken { token: t, expecting: String::from("Expression")}),
            };
        }

        Err(ParseError::UnexpectedEnd)
    }

    // With statements, we allow an EOF on the first token (returns None), as we can't know for sure if there
    // will be a next statement or not.
    fn try_parse_statement(&mut self) -> Option<Result<ast::Statement, ParseError>> {
        if let Some(token) = self.next_token() {
            let statement = match token {
                Token::Let => Some(self.parse_let_statement()),
                Token::Return => Some(self.parse_return_statement()),
                t => {
                    // Try to parse expression as ExpressionStatement
                    self.save_token(t);
                    match self.parse_expression() {
                        Ok(expression) => Some(Ok(ast::Statement::ExpressionStatement{expression})),
                        Err(e) => Some(Err(e))
                    }
                }
            };

            if let Err(e) = self.skip_token_expecting(Token::Semicolon) {
                return Some(Err(e));
            }

            return statement;
        }

        None
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program: ast::Program = Vec::new();
        loop {
            match self.try_parse_statement() {
                None => break,
                Some(Ok(statement)) => program.push(statement),
                Some(Err(e)) => return Err(e)
            };
        }

        Ok(program)
    }
}
