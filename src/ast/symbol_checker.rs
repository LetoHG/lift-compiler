use crate::diagnostics::DiagnosticsColletionCell;

use super::ASTVisitor;

pub struct SymbolChecker {
    symbols: Vec<String>,
    diagnostics: DiagnosticsColletionCell,
}

impl SymbolChecker {
    pub fn new(diagnostics: DiagnosticsColletionCell) -> Self {
        Self {
            symbols: Vec::new(),
            diagnostics,
        }
    }
}

impl ASTVisitor for SymbolChecker {
    fn visit_return_statement(&mut self, statement: &super::ASTReturnStatement) {
        self.visit_expression(&statement.expr);
    }

    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        self.symbols.push(statement.identifier.span.literal.clone());
        self.visit_expression(&statement.initializer);
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        if !self.symbols.contains(&expr.identifier().to_string()) {
            self.diagnostics
                .borrow_mut()
                .report_undefined_variable(expr.identifier.span.clone());
        }
        for arg in expr.arguments.iter() {
            self.visit_expression(arg);
        }
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        if !self.symbols.contains(&expr.identifier().to_string()) {
            self.diagnostics
                .borrow_mut()
                .report_undefined_variable(expr.identifier.span.clone());
        }
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.visit_expression(&expr.expr);
    }

    fn visit_binary_operator(&mut self, op: &super::ASTBinaryOperator) {}
    fn visit_integer(&mut self, integer: &i64) {}
    fn visit_float(&mut self, float: &f64) {}
}
