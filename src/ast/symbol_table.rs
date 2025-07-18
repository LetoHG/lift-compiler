use std::collections::HashMap;

use crate::diagnostics::DiagnosticsColletionCell;

use super::ASTVisitor;

#[derive(Debug)]
struct VariableInfo {
    name: String,
    data_type: String,
}

#[derive(Debug)]
struct FunctionInfo {
    name: String,
    parameters: Vec<String>, // Simplified
    return_type: String,
}

enum Symbol {
    Variable(VariableInfo),
    Constant(VariableInfo),
    Function(FunctionInfo),
}
impl Symbol {
    pub fn name(&self) -> String {
        match &self {
            Symbol::Variable(a) => a.name.clone(),
            Symbol::Constant(a) => a.name.clone(),
            Symbol::Function(a) => a.name.clone(),
        }
    }
}

#[derive(Debug)]
enum Pass {
    CollectSymbols,
    TypeCheck,
}

pub struct SymbolTable {
    symbol_table: HashMap<String, Symbol>,
    global_scope: HashMap<String, Symbol>,
    scopes: Vec<HashMap<String, Symbol>>,
    pass: Pass,
    diagnostics: DiagnosticsColletionCell,
}

impl SymbolTable {
    pub fn new(diagnostics: DiagnosticsColletionCell) -> Self {
        Self {
            symbol_table: HashMap::new(),
            global_scope: HashMap::new(),
            scopes: Vec::new(),
            pass: Pass::CollectSymbols,
            diagnostics,
        }
    }
    pub fn build(&mut self, ast: &super::Ast) {
        ast.visit(self);
        self.pass = Pass::TypeCheck;
        ast.visit(self);
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_global_identifier(&mut self, symbol: Symbol) {
        if self.global_scope.contains_key(&symbol.name()) {
            // TODO(letohg): [2025-07-18] output diagnostic message
            // return Err(format!("Redefinition of global symbol `{}`", symbol.name));
        }

        self.global_scope.insert(symbol.name(), symbol);
    }
    fn declare_local_identifier(&mut self, symbol: Symbol) {
        let scope = self.scopes.last_mut().expect("No scope available");
        if scope.contains_key(&symbol.name()) {
            // TODO(letohg): [2025-07-18] output diagnostic message
            // return Err(format!("Redefinition of symbol `{}`", symbol.name));
        }
        scope.insert(symbol.name(), symbol);
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        self.global_scope.get(name)
    }
}

impl ASTVisitor<()> for SymbolTable {
    fn visit_return_statement(&mut self, statement: &super::ASTReturnStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.diagnostics.borrow_mut().report_error(
                    format!("Return statement not allowed outside of functions"),
                    super::lexer::TextSpan {
                        start: 0,
                        end: 0,
                        literal: "if".to_string(),
                    },
                );
            }
            Pass::TypeCheck => {
                self.visit_expression(&statement.expr);
            }
        }
    }

    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.declare_global_identifier(Symbol::Constant(VariableInfo {
                    name: statement.identifier.name(),
                    data_type: statement.data_type.name(),
                }));
            }
            Pass::TypeCheck => {
                self.declare_local_identifier(Symbol::Constant(VariableInfo {
                    name: statement.identifier.name(),
                    data_type: statement.data_type.name(),
                }));
                self.visit_expression(&statement.initializer);
            }
        }
    }

    fn visit_var_statement(&mut self, statement: &super::ASTVarStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.declare_global_identifier(Symbol::Variable(VariableInfo {
                    name: statement.identifier.name(),
                    data_type: statement.data_type.name(),
                }));
            }
            Pass::TypeCheck => {
                self.declare_local_identifier(Symbol::Variable(VariableInfo {
                    name: statement.identifier.name(),
                    data_type: statement.data_type.name(),
                }));
                self.visit_expression(&statement.initializer);
            }
        }
    }

    fn visit_compound_statement(&mut self, statement: &super::ASTCompoundStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.diagnostics.borrow_mut().report_error(
                    format!("standalone Compound statement not allowed outside of functions"),
                    super::lexer::TextSpan {
                        start: 0,
                        end: 0,
                        literal: "if".to_string(),
                    },
                );
            }

            Pass::TypeCheck => {
                self.enter_scope();
                for statement in statement.statements.iter() {
                    self.visit_statement(statement);
                }
                self.exit_scope();
            }
        }
    }

    fn visit_if_statement(&mut self, statement: &super::ASTIfStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.diagnostics.borrow_mut().report_error(
                    format!("If statement not allowed outside of functions"),
                    super::lexer::TextSpan {
                        start: 0,
                        end: 0,
                        literal: "if".to_string(),
                    },
                );
            }
            Pass::TypeCheck => {}
        }
    }

    fn visit_for_loop_statement(&mut self, statement: &super::ASTForStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.diagnostics.borrow_mut().report_error(
                    format!("For loop statement not allowed outside of functions"),
                    super::lexer::TextSpan {
                        start: 0,
                        end: 0,
                        literal: "if".to_string(),
                    },
                );
            }
            Pass::TypeCheck => {}
        }
    }

    fn visit_while_loop_statement(&mut self, statement: &super::ASTWhileStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                self.diagnostics.borrow_mut().report_error(
                    format!("While statement not allowed outside of functions"),
                    super::lexer::TextSpan {
                        start: 0,
                        end: 0,
                        literal: "if".to_string(),
                    },
                );
            }
            Pass::TypeCheck => {}
        }
    }

    fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
        match self.pass {
            Pass::CollectSymbols => {
                let mut arguments_names: Vec<String> = Vec::new();
                // add arguments to scope of local variable call
                for arg in function.arguments.iter() {
                    // arguments_names.push(arg.identifier.span.literal.clone());
                    arguments_names.push(arg.data_type.name());
                }
                self.symbol_table.insert(
                    function.identifier.name(),
                    Symbol::Function(FunctionInfo {
                        name: function.identifier.name(),
                        parameters: arguments_names.clone(),
                        return_type: function.return_type.span.literal.clone(),
                    }),
                );
            }
            Pass::TypeCheck => {
                self.visit_statement(&function.body);
            }
        }
    }

    fn visit_assignment_expression(&mut self, expr: &super::ASTAssignmentExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                if self.lookup(&expr.identifier.name().to_string()).is_none() {
                    self.diagnostics
                        .borrow_mut()
                        .report_undefined_variable(expr.identifier.span.clone());
                }
                self.visit_expression(&expr.expr)
            }
        }
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                if !self
                    .symbol_table
                    .contains_key(&expr.identifier().to_string())
                {
                    self.diagnostics
                        .borrow_mut()
                        .report_undefined_function(expr.identifier.span.clone());
                    return;
                }

                let expected_number_of_arguments =
                    match self.symbol_table.get(expr.identifier()).unwrap() {
                        Symbol::Function(func) => func.parameters.len(),
                        _ => {
                            println!("Not a callable!");
                            return;
                        }
                    };

                if expected_number_of_arguments != expr.arguments.len() {
                    self.diagnostics
                        .borrow_mut()
                        .report_number_of_function_arguments_mismatch(
                            expr.identifier.span.clone(),
                            expected_number_of_arguments,
                            expr.arguments.len(),
                        );
                    return;
                }

                for arg in expr.arguments.iter() {
                    self.visit_expression(arg);
                }
            }
        }
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                if self.lookup(&expr.identifier().to_string()).is_none() {
                    self.diagnostics
                        .borrow_mut()
                        .report_undefined_variable(expr.identifier.span.clone());
                }
            }
        }
    }

    fn visit_unary_expression(&mut self, expr: &super::ASTUnaryExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                self.visit_expression(&expr.expr);
            }
        }
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                self.visit_expression(&expr.left);
                self.visit_expression(&expr.right);
            }
        }
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {
                self.visit_expression(&expr.expr);
            }
        }
    }

    fn visit_binary_operator(&mut self, op: &super::ASTBinaryOperator) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {}
        }
    }
    fn visit_error(&mut self, span: &super::lexer::TextSpan) -> () {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {}
        }
    }
    fn visit_integer(&mut self, integer: &i64) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {}
        }
    }
    fn visit_float(&mut self, float: &f64) {
        match self.pass {
            Pass::CollectSymbols => {}
            Pass::TypeCheck => {}
        }
    }
}
