use std::fmt::Arguments;
use std::{collections::HashMap, ops::Not};

use super::{
    ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpressionKind, ASTFunctionStatement,
    ASTReturnStatement, ASTStatementKind, ASTVisitor,
};

type Scope = HashMap<String, f64>;
pub struct ASTSolver {
    result: Option<f64>,
    scopes: Vec<Scope>,
    functions: HashMap<String, ASTFunctionStatement>,
}

impl ASTSolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
            result: None,
            functions: HashMap::new(),
        }
    }

    pub fn print_result(&self) {
        println!("Solver result: {}", self.result.unwrap());
    }

    fn enter_scope(&mut self, scope_variables: Scope) {
        self.scopes.push(scope_variables);
        // self.active_scope += 1;
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
        // self.active_scope -= 1;
    }

    fn add_identifier_to_scope(&mut self, identifier: &String, value: f64) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(identifier.clone(), value);
    }

    fn check_identifier_in_scope(&self, identifier: &String) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(identifier) {
                return true;
            }
        }
        return false;
    }

    fn get_identifier_in_scope(&self, identifier: &String) -> Option<f64> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(identifier) {
                return scope.get(identifier).copied();
            }
        }
        return None;
    }
}

impl ASTVisitor for ASTSolver {
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement) {
        self.visit_expression(&statement.expr);
    }
    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        self.visit_expression(&statement.initializer);
        self.add_identifier_to_scope(&statement.identifier.span.literal, self.result.unwrap());
    }

    fn visit_conditional_statement(&mut self, statement: &super::ASTConditionalStatement) {
        self.visit_expression(&statement.condition);
        let condition = self.result.unwrap();

        if condition != 0.0 {
            self.visit_statement(&statement.then_branch);
        } else if let Some(else_branch) = &statement.else_branch {
            self.visit_statement(&else_branch.else_branch);
        }
    }

    fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
        self.functions
            .insert(function.identifier.span.literal.clone(), function.clone());

        self.add_identifier_to_scope(&function.identifier.span.literal, 0.0);
    }

    fn visit_assignment_expression(&mut self, expr: &super::ASTAssignmentExpression) {
        self.visit_expression(&expr.expr);
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.get_mut(&expr.identifier.span.literal) {
                *value = self.result.unwrap();
            }
        }
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        if !self.check_identifier_in_scope(&expr.identifier.span.literal) {}

        let func = self
            .functions
            .get(&expr.identifier.span.literal)
            .unwrap()
            .clone();
        let mut arguments: Scope = Scope::new();

        // evaluate arguments and add them to scope
        // arguments.push(expr.identifier.span.literal.clone());
        for (arg_expr, func_arg) in expr.arguments.iter().zip(func.arguments.iter()) {
            self.visit_expression(&arg_expr);
            let arg_name = func_arg.identifier.span.literal.clone();

            arguments.insert(arg_name, self.result.unwrap());
        }
        self.enter_scope(arguments);

        // todo: that check should be done before
        if let super::ASTStatementKind::CompoundStatement(statement) = &func.body.kind {
            for statement in statement.statements.iter() {
                self.visit_statement(statement);
            }
        }

        self.leave_scope();
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.result = self.get_identifier_in_scope(&expr.identifier.span.literal);
        // self.result = Some(*self.variables.get(expr.identifier()).unwrap());
    }

    fn visit_unary_expression(&mut self, expr: &super::ASTUnaryExpression) {
        self.visit_expression(&expr.expr);
        self.result = Some(match expr.operator.kind {
            super::ASTUnaryOperatorKind::BitwiseNOT => (self.result.unwrap() as i64).not() as f64,
            super::ASTUnaryOperatorKind::LogicNot => ((self.result.unwrap() == 0.0) as i64) as f64,
            super::ASTUnaryOperatorKind::Minus => self.result.unwrap() * -1.0,
        });
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
            ASTBinaryOperatorKind::EqualTo => (left == right) as i64 as f64,
            ASTBinaryOperatorKind::NotEqualTo => (left != right) as i64 as f64,
            ASTBinaryOperatorKind::LogicAND => ((left != 0.0) && (right != 0.0)) as i64 as f64,
            ASTBinaryOperatorKind::LogicOR => ((left != 0.0) || (right != 0.0)) as i64 as f64,
            ASTBinaryOperatorKind::GreaterThan => (left > right) as i64 as f64,
            ASTBinaryOperatorKind::GreaterThanOrEqual => (left >= right) as i64 as f64,
            ASTBinaryOperatorKind::LessThan => (left < right) as i64 as f64,
            ASTBinaryOperatorKind::LessThanOrEqual => (left <= right) as i64 as f64,
            ASTBinaryOperatorKind::BitwiseOR => ((left as i64) | (right as i64)) as f64,
            ASTBinaryOperatorKind::BitwiseAND => ((left as i64) & (right as i64)) as f64,
            ASTBinaryOperatorKind::BitwiseXOR => ((left as i64) ^ (right as i64)) as f64,
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
