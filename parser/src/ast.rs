#[derive(PartialEq, Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement {
        identifier: Identifier,
        expression: Expression,
    },
    ReturnStatement {
        expression: Expression,
    },
    ExpressionStatement {
        expression: Expression,
    },
}

#[derive(PartialEq, Debug)]
pub enum InfixOperation {
    Plus,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    IntegerLiteral {
        value: i64,
    },
    InfixExpression {
        operation: InfixOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

pub type Program = Vec<Statement>;
