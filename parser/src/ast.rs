#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    LetStatement {
        identifier: String,
        expression: Expression,
    },
    ReturnStatement {
        expression: Expression,
    },
    ExpressionStatement {
        expression: Expression,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum InfixOperation {
    Sum,
    Product,
    Division,
    Subtraction,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PrefixOperation {
    Negative,
    Negate,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    IntegerLiteral {
        value: i64,
    },
    IdentifierExpression {
        identifier: String,
    },
    PrefixExpression {
        operation: PrefixOperation,
        right: Box<Expression>,
    },
    InfixExpression {
        operation: InfixOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    Boolean {
        value: bool,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

pub type Program = Vec<Statement>;
