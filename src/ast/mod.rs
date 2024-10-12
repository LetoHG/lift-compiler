use lexer::{TextSpan, Token};
use nerd_font_symbols::seti::SETI_AUDIO;
use termion::color;

pub mod lexer;
pub mod parser;
pub mod printer;
pub mod solver;

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
        let mut printer = ASTPrinter { indentation: 0 };
        println!("AST:");
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
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
            ASTExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn visit_let_statement(&mut self, statement: &ASTLetStatement);

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

pub struct ASTPrinter {
    indentation: usize,
}

impl ASTPrinter {
    const INDENATION: usize = 2;

    const TEXT_COLOR: color::White = color::White;
    const STATEMENT_COLOR: color::Yellow = color::Yellow;
    const LET_STATEMENT_COLOR: color::Green = color::Green;
    const EXPR_COLOR: color::Green = color::Green;
    const BIN_EXPR_COLOR: color::LightBlue = color::LightBlue;
    const OPERATOR_COLOR: color::LightYellow = color::LightYellow;

    const STATEMENT_ICON: &str = nerd_font_symbols::md::MD_SIGMA;
    const LET_STATEMENT_ICON: &str = nerd_font_symbols::md::MD_EQUAL;
    const EXPR_ICON: &str = nerd_font_symbols::md::MD_FUNCTION_VARIANT;
    const BIN_EXPR_ICON: &str = nerd_font_symbols::cod::COD_SYMBOL_OPERATOR;
    const VARIABLE_ICON: &str = nerd_font_symbols::md::MD_VARIABLE;

    fn increase_indentation(&mut self) {
        self.indentation += Self::INDENATION;
    }
    fn decrease_indentation(&mut self) {
        self.indentation -= Self::INDENATION;
    }

    fn print(&self, text: &str, text_color: &dyn color::Color) {
        // println!("{}├─ {}", "│ ".repeat(self.indentation), text);
        println!(
            "{}└─ {}{}{}",
            " ".repeat(self.indentation),
            color::Fg(text_color),
            text,
            color::Fg(color::Reset)
        );
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print(
            &format!("{}  Statement:", Self::STATEMENT_ICON),
            &Self::STATEMENT_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_statement(self, statement);
        self.decrease_indentation();
    }

    fn visit_let_statement(&mut self, statement: &ASTLetStatement) {
        self.print(
            &format!(
                "{}  Declaration: {}{}",
                Self::LET_STATEMENT_ICON,
                color::Fg(Self::TEXT_COLOR),
                &statement.identifier.span.literal
            ),
            &Self::LET_STATEMENT_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &statement.initializer);
        self.decrease_indentation();
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.print(
            &format!("{}  Expression:", Self::EXPR_ICON),
            &Self::EXPR_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &expr);
        self.decrease_indentation();
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.print(
            &format!(
                "{}  Binary: {}{}",
                Self::BIN_EXPR_ICON,
                color::Fg(Self::OPERATOR_COLOR),
                expr.operator.token.span.literal
            ),
            &Self::BIN_EXPR_COLOR,
        );
        self.increase_indentation();
        // self.print_binary_operator(&expr.operator);
        // self.print(&format!("{:?}", expr.operator.kind), &Self::TEXT_COLOR);
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
        self.decrease_indentation();
    }

    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator) {
        self.print(
            &format!(
                "Operator: {}",
                match op.kind {
                    ASTBinaryOperatorKind::Plus => '+',
                    ASTBinaryOperatorKind::Minus => '-',
                    ASTBinaryOperatorKind::Multiply => '*',
                    ASTBinaryOperatorKind::Divide => '/',
                }
            ),
            &color::Yellow,
        );
    }

    fn visit_parenthesised_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.print(
            &format!(
                "{}  Parenthesized:",
                nerd_font_symbols::md::MD_CODE_PARENTHESES
            ),
            &color::Magenta,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &expr.expr);
    }

    fn visit_variable_expression(&mut self, expr: &ASTVariableExpression) {
        self.print(
            &format!("{}  Variable: {}", Self::VARIABLE_ICON, expr.identifier()),
            &Self::TEXT_COLOR,
        );
    }
    fn visit_error(&mut self, span: &TextSpan) {
        self.print(&format!("Error: {:?}", span), &color::Red);
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.print(&format!("Integer: {}", integer), &Self::TEXT_COLOR);
    }

    fn visit_float(&mut self, float: &f64) {
        self.print(&format!("Float: {}", float), &Self::TEXT_COLOR);
    }
}

enum ASTStatementKind {
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
}

pub struct ASTLetStatement {
    identifier: Token,
    initializer: ASTExpression,
}

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

    fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::LetStatement(ASTLetStatement {
                identifier: identifier,
                initializer,
            }),
        }
    }
}

enum ASTExpressionKind {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    StringLiteral(String),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Variable(ASTVariableExpression),
    Error(TextSpan),
}

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
}

#[derive(Debug)]
enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
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

pub struct ASTBinaryExpression {
    operator: ASTBinaryOperator,
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
}

pub struct ASTParenthesizedExpression {
    expr: Box<ASTExpression>,
}

pub struct ASTVariableExpression {
    identifier: Token,
}

impl ASTVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}
