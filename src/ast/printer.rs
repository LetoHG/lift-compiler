use super::{
    ASTBinaryOperator, ASTBinaryOperatorKind, ASTLetStatement, ASTReturnStatement, ASTVisitor,
};

use termion::color::Fg;
use termion::color::{self, White};
pub struct ASTHiglightPrinter {
    indent: usize,
    result: String,
}

impl ASTHiglightPrinter {
    const INTEGER_COLOR: color::Cyan = color::Cyan;
    const FLOAT_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::White = color::White;
    const LET_COLOR: color::Red = color::Red;
    const FUNC_COLOR: color::Yellow = color::Yellow;
    const VARIABLE_COLOR: color::LightGreen = color::LightGreen;

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

    fn visit_idenifier(&mut self, identifier: &String) {
        self.result
            .push_str(&format!("{}{}", Fg(Self::TEXT_COLOR), identifier));
    }
}

impl ASTVisitor for ASTHiglightPrinter {
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement) {
        self.result
            .push_str(&format!("{}return", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.expr);
    }

    fn visit_let_statement(&mut self, statement: &ASTLetStatement) {
        self.result.push_str(&format!("{}let", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_idenifier(&statement.identifier.span.literal);
        self.add_whitespace();
        self.result.push_str(&format!("{}=", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.initializer);
        self.add_newline();
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        self.result.push_str(&format!(
            "{}{}{}(",
            Fg(Self::FUNC_COLOR),
            expr.identifier(),
            Fg(Self::TEXT_COLOR)
        ));

        for (i, arg) in expr.arguments.iter().enumerate() {
            self.visit_expression(arg);
            if (i + 1) < expr.arguments.len() {
                self.result.push_str(&format!("{}, ", Fg(Self::TEXT_COLOR)));
                self.add_whitespace();
            }
        }
        self.result.push_str(&format!("{})", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.result.push_str(&format!(
            "{}{}",
            Fg(Self::VARIABLE_COLOR),
            expr.identifier()
        ));
    }

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
