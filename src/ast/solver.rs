use std::collections::HashMap;

use super::{ASTBinaryOperator, ASTBinaryOperatorKind, ASTReturnStatement, ASTVisitor};

use termion::color;
use termion::color::Fg;
pub struct ASTSolver {
    result: Option<f64>,
    variables: HashMap<String, f64>,
}

impl ASTSolver {
    pub fn new() -> Self {
        Self {
            result: None,
            variables: HashMap::new(),
        }
    }

    pub fn print_result(&self) {
        println!("Solver result: {}", self.result.unwrap());
    }
}

impl ASTVisitor for ASTSolver {
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement) {
        self.visit_expression(&statement.expr);
    }
    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        self.visit_expression(&statement.initializer);
        self.variables.insert(
            statement.identifier.span.literal.clone(),
            self.result.unwrap(),
        );
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        todo!();
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.result = Some(*self.variables.get(expr.identifier()).unwrap());
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.result.unwrap();
        self.visit_expression(&expr.right);
        let right = self.result.unwrap();
        self.result = Some(match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        })
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.visit_expression(&expr.expr);
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.result = Some(integer.clone() as f64);
    }
    fn visit_float(&mut self, float: &f64) {
        self.result = Some(float.clone());
    }

    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator) {}
}
