use lexer::{TextSpan, Token};
use printer::ASTTreePrinter;

pub mod lexer;
pub mod parser;
pub mod printer;
pub mod solver;
pub mod symbol_checker;

pub struct Ast {
    statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, printer: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            printer.visit_statement(statement);
        }
    }

    pub fn visualize(&self) {
        let mut printer = ASTTreePrinter::new();
        println!("AST:");
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
            ASTStatementKind::ReturnStatement(statement) => self.visit_return_statement(statement),
            ASTStatementKind::FunctionStatement(statement) => {
                self.visit_funtion_statement(statement)
            }
            ASTStatementKind::LetStatement(statement) => self.visit_let_statement(statement),
        }
    }

    fn do_visit_expression(&mut self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::IntegerLiteral(i) => self.visit_integer(i),
            ASTExpressionKind::FloatingLiteral(f) => self.visit_float(f),
            ASTExpressionKind::Variable(expr) => self.visit_variable_expression(expr),
            ASTExpressionKind::StringLiteral(_) => todo!(),
            ASTExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ASTExpressionKind::Parenthesized(expr) => self.visit_parenthesised_expression(expr),
            ASTExpressionKind::FunctionCall(expr) => self.visit_function_call_expression(expr),
            ASTExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement);
    fn visit_let_statement(&mut self, statement: &ASTLetStatement);
    fn visit_funtion_statement(&mut self, function: &ASTFunctionStatement) {
        for arg in function.arguments.iter() {
            self.visit_expression(arg);
        }
        for statement in function.body.iter() {
            self.visit_statement(statement);
        }
    }

    fn visit_function_call_expression(&mut self, expr: &ASTFunctionCallExpression);

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_variable_expression(&mut self, expr: &ASTVariableExpression);
    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression);
    fn visit_parenthesised_expression(&mut self, expr: &ASTParenthesizedExpression);
    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator);

    fn visit_error(&mut self, span: &TextSpan) {}

    fn visit_integer(&mut self, integer: &i64);
    fn visit_float(&mut self, float: &f64);
}

#[derive(Clone)]
enum ASTStatementKind {
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
    ReturnStatement(ASTReturnStatement),
    FunctionStatement(ASTFunctionStatement),
}

#[derive(Clone)]
pub struct ASTLetStatement {
    identifier: Token,
    initializer: ASTExpression,
}

#[derive(Clone)]
pub struct ASTReturnStatement {
    expr: ASTExpression,
}
#[derive(Clone)]
pub struct ASTFunctionStatement {
    identifier: Token,
    arguments: Vec<ASTExpression>,
    body: Vec<ASTStatement>,
}

#[derive(Clone)]
pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    fn new(kind: ASTStatementKind) -> Self {
        Self { kind }
    }

    fn expression(expr: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::Expression(expr),
        }
    }

    fn return_statement(expr: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::ReturnStatement(ASTReturnStatement { expr }),
        }
    }
    fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::LetStatement(ASTLetStatement {
                identifier: identifier,
                initializer,
            }),
        }
    }

    fn function(identifier: Token, arguments: Vec<ASTExpression>, body: Vec<ASTStatement>) -> Self {
        Self {
            kind: ASTStatementKind::FunctionStatement(ASTFunctionStatement {
                identifier: identifier,
                arguments,
                body,
            }),
        }
    }
}

#[derive(Clone)]
enum ASTExpressionKind {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    StringLiteral(String),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Variable(ASTVariableExpression),
    FunctionCall(ASTFunctionCallExpression),
    Error(TextSpan),
}

#[derive(Clone)]
pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    fn new(kind: ASTExpressionKind) -> Self {
        Self { kind }
    }
    fn error(span: TextSpan) -> Self {
        Self {
            kind: ASTExpressionKind::Error(span),
        }
    }

    fn integer(i: i64) -> Self {
        Self {
            kind: ASTExpressionKind::IntegerLiteral(i),
        }
    }
    fn float(f: f64) -> Self {
        Self {
            kind: ASTExpressionKind::FloatingLiteral(f),
        }
    }

    fn identifier(token: Token) -> Self {
        Self {
            kind: ASTExpressionKind::Variable(ASTVariableExpression { identifier: token }),
        }
    }

    fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        Self {
            kind: ASTExpressionKind::Binary(ASTBinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            }),
        }
    }

    fn parenthesized(expr: ASTExpression) -> Self {
        Self {
            kind: ASTExpressionKind::Parenthesized(ASTParenthesizedExpression {
                expr: Box::new(expr),
            }),
        }
    }

    fn function_call(identifier: Token, arguments: Vec<ASTExpression>) -> Self {
        Self {
            kind: ASTExpressionKind::FunctionCall(ASTFunctionCallExpression {
                identifier,
                arguments,
            }),
        }
    }
}

#[derive(Debug, Clone)]
enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: lexer::Token,
}

impl ASTBinaryOperator {
    fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus => 1,
            ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply => 2,
            ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

#[derive(Clone)]
pub struct ASTBinaryExpression {
    operator: ASTBinaryOperator,
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
}

#[derive(Clone)]
pub struct ASTParenthesizedExpression {
    expr: Box<ASTExpression>,
}

#[derive(Clone)]
pub struct ASTVariableExpression {
    identifier: Token,
}

impl ASTVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}

#[derive(Clone)]
pub struct ASTFunctionCallExpression {
    identifier: Token,
    arguments: Vec<ASTExpression>,
}

impl ASTFunctionCallExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}
