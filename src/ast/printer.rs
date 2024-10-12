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
    const INDENATION: usize = 2;
    const INTEGER_COLOR: color::Cyan = color::Cyan;
    const FLOAT_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::White = color::White;
    const LET_COLOR: color::Red = color::Red;
    const FUNC_COLOR: color::Red = color::Red;
    const FUNC_CALL_COLOR: color::Yellow = color::Yellow;
    const FUNC_NAME_COLOR: color::Yellow = color::Yellow;
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
        self.print(" ");
    }

    fn add_newline(&mut self) {
        self.print("\n");
    }

    fn increase_indentation(&mut self) {
        self.indent += Self::INDENATION;
    }
    fn decrease_indentation(&mut self) {
        self.indent -= Self::INDENATION;
    }

    fn print(&mut self, text: &str) {
        self.result.push_str(&format!(
            "{}{}{}",
            " ".repeat(self.indent),
            text,
            color::Fg(color::Reset)
        ));
    }

    fn visit_idenifier(&mut self, identifier: &String) {
        self.print(&format!("{}{}", Fg(Self::TEXT_COLOR), identifier));
    }
}

impl ASTVisitor for ASTHiglightPrinter {
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement) {
        self.print(&format!("{}return", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.expr);
    }

    fn visit_let_statement(&mut self, statement: &ASTLetStatement) {
        self.print(&format!("{}let", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_idenifier(&statement.identifier.span.literal);
        self.add_whitespace();
        self.print(&format!("{}=", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.initializer);
        self.add_newline();
    }

    fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
        self.print(&format!(
            "{}func {}{}{}(",
            Fg(Self::FUNC_COLOR),
            Fg(Self::FUNC_NAME_COLOR),
            function.identifier.span.literal,
            Fg(Self::TEXT_COLOR),
        ));
        for arg in function.arguments.iter() {
            self.visit_expression(arg);
        }
        self.print(&format!("{}) {}", Fg(Self::TEXT_COLOR), '{'));
        self.add_newline();
        for statement in function.body.iter() {
            self.print("  ");
            self.visit_statement(statement);
        }
        self.add_newline();
        self.print(&format!("{}{}", Fg(Self::TEXT_COLOR), '}'));
        self.add_newline();
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        self.print(&format!(
            "{}{}{}(",
            Fg(Self::FUNC_CALL_COLOR),
            expr.identifier(),
            Fg(Self::TEXT_COLOR)
        ));

        for (i, arg) in expr.arguments.iter().enumerate() {
            self.visit_expression(arg);
            if (i + 1) < expr.arguments.len() {
                self.print(&format!("{}, ", Fg(Self::TEXT_COLOR)));
                self.add_whitespace();
            }
        }
        self.print(&format!("{})", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.print(&format!(
            "{}{}",
            Fg(Self::VARIABLE_COLOR),
            expr.identifier()
        ));
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        self.add_whitespace();
        self.print(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            expr.operator.token.span.literal
        ));
        self.add_whitespace();
        self.visit_expression(&expr.right);
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.print(&format!("{}(", Fg(Self::TEXT_COLOR)));
        self.visit_expression(&expr.expr);
        self.print(&format!("{})", Fg(Self::TEXT_COLOR)));
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.print(&format!("{}{}", Fg(Self::INTEGER_COLOR), integer));
    }
    fn visit_float(&mut self, float: &f64) {
        self.print(&format!("{}{}", Fg(Self::FLOAT_COLOR), float));
    }

    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator) {
        self.print(&format!(
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
