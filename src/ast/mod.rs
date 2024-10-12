use lexer::TextSpan;
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
        }
    }

    fn do_visit_expression(&mut self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::IntegerLiteral(i) => self.visit_integer(i),
            ASTExpressionKind::FloatingLiteral(f) => self.visit_float(f),
            ASTExpressionKind::StringLiteral(_) => todo!(),
            ASTExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ASTExpressionKind::Parenthesized(expr) => self.visit_parenthesised_expression(expr),
            ASTExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

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

const INDENATION: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print(
            &format!("{}  Statement:", nerd_font_symbols::md::MD_SIGMA),
            &color::Yellow,
        );
        self.indentation += INDENATION;
        ASTVisitor::do_visit_statement(self, statement);
        self.indentation -= INDENATION;
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.print(
            &format!(
                "{}  Expression:",
                nerd_font_symbols::md::MD_FUNCTION_VARIANT
            ),
            &color::Green,
        );
        self.indentation += INDENATION;
        ASTVisitor::do_visit_expression(self, &expr);
        self.indentation -= INDENATION;
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.print(
            &format!(
                "{}  Binary: {}{}",
                nerd_font_symbols::cod::COD_SYMBOL_OPERATOR,
                color::Fg(color::LightYellow),
                expr.operator.token.span.literal
            ),
            &color::LightBlue,
        );
        self.indentation += INDENATION;
        // self.print_binary_operator(&expr.operator);
        // self.print(&format!("{:?}", expr.operator.kind), &color::White);
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
        self.indentation -= INDENATION;
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
        self.indentation += INDENATION;
        ASTVisitor::do_visit_expression(self, &expr.expr);
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.print(&format!("Error: {:?}", span), &color::Red);
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.print(
            &format!("Integer: {}{}", color::Fg(color::Blue), integer),
            &color::White,
        );
    }

    fn visit_float(&mut self, float: &f64) {
        self.print(
            &format!("Float: {}{}", color::Fg(color::Blue), float),
            &color::White,
        );
    }
}

impl ASTPrinter {
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

enum ASTStatementKind {
    Expression(ASTExpression),
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
}

enum ASTExpressionKind {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    StringLiteral(String),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
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
