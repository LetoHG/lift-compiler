use super::{ASTBinaryOperator, ASTBinaryOperatorKind, ASTVisitor};

use termion::color::Fg;
use termion::color::{self, White};
pub struct ASTHiglightPrinter {
    indent: usize,
    result: String,
}

impl ASTHiglightPrinter {
    const INTEGER_COLOR: color::Cyan = color::Cyan;
    const FLOAT_COLOR: color::Green = color::Green;
    const TEXT_COLOR: color::White = color::White;

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: "".to_string(),
        }
    }

    pub fn print_result(&self) {
        println!("Highlighted Source:\n{}{}", self.result, Fg(White));
    }

    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_newline(&mut self) {
        self.result.push_str("\n");
    }
}

impl ASTVisitor for ASTHiglightPrinter {
    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        self.add_whitespace();
        self.result.push_str(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            expr.operator.token.span.literal
        ));
        self.add_whitespace();
        self.visit_expression(&expr.right);
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.result.push_str(&format!("{}(", Fg(Self::TEXT_COLOR)));
        self.visit_expression(&expr.expr);
        self.result.push_str(&format!("{})", Fg(Self::TEXT_COLOR)));
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.result
            .push_str(&format!("{}{}", Fg(Self::INTEGER_COLOR), integer));
    }
    fn visit_float(&mut self, float: &f64) {
        self.result
            .push_str(&format!("{}{}", Fg(Self::FLOAT_COLOR), float));
    }

    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator) {
        self.result.push_str(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            match op.kind {
                ASTBinaryOperatorKind::Plus => '+',
                ASTBinaryOperatorKind::Minus => '-',
                ASTBinaryOperatorKind::Multiply => '*',
                ASTBinaryOperatorKind::Divide => '/',
            }
        ));
    }
}
