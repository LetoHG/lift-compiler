use super::{ASTBinaryOperator, ASTBinaryOperatorKind, ASTVisitor};

use termion::color;
use termion::color::Fg;
pub struct ASTSolver {
    result: f64,
}

impl ASTSolver {
    pub fn new() -> Self {
        Self { result: 0.0 }
    }

    pub fn print_result(&self) {
        println!("Solver result: {}", self.result);
    }
}

impl ASTVisitor for ASTSolver {
    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.result;
        self.visit_expression(&expr.right);
        let right = self.result;
        self.result = match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        }
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.visit_expression(&expr.expr);
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.result = integer.clone() as f64;
    }
    fn visit_float(&mut self, float: &f64) {
        self.result = float.clone();
    }

    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator) {}
}
