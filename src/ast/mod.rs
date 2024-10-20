use lexer::{TextSpan, Token};
use printer::ASTTreePrinter;

pub mod lexer;
pub mod parser;
pub mod printer;
pub mod solver;
pub mod symbol_checker;

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
        let mut printer = ASTTreePrinter::new();
        println!("AST:");
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
            ASTStatementKind::ReturnStatement(statement) => self.visit_return_statement(statement),
            ASTStatementKind::FunctionStatement(statement) => {
                self.visit_funtion_statement(statement)
            }
            ASTStatementKind::LetStatement(statement) => self.visit_let_statement(statement),
            ASTStatementKind::CompoundStatement(statement) => {
                self.visit_compound_statement(statement)
            }
            ASTStatementKind::ConditionalStatement(statement) => {
                self.visit_conditional_statement(statement)
            }
        }
    }

    fn do_visit_expression(&mut self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::IntegerLiteral(i) => self.visit_integer(i),
            ASTExpressionKind::FloatingLiteral(f) => self.visit_float(f),
            ASTExpressionKind::Variable(expr) => self.visit_variable_expression(expr),
            ASTExpressionKind::StringLiteral(_) => todo!(),
            ASTExpressionKind::Unary(expr) => self.visit_unary_expression(expr),
            ASTExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ASTExpressionKind::Parenthesized(expr) => self.visit_parenthesised_expression(expr),
            ASTExpressionKind::FunctionCall(expr) => self.visit_function_call_expression(expr),
            ASTExpressionKind::Assignment(expr) => self.visit_assignment_expression(expr),
            ASTExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn visit_return_statement(&mut self, statement: &ASTReturnStatement);
    fn visit_let_statement(&mut self, statement: &ASTLetStatement);
    fn visit_compound_statement(&mut self, statement: &ASTCompoundStatement) {
        for statement in statement.statements.iter() {
            self.visit_statement(statement);
        }
    }
    fn visit_conditional_statement(&mut self, statement: &ASTConditionalStatement);
    fn visit_funtion_statement(&mut self, function: &ASTFunctionStatement) {
        if let ASTStatementKind::CompoundStatement(statement) = &function.body.kind {
            self.visit_compound_statement(statement);
        }
    }

    fn visit_function_call_expression(&mut self, expr: &ASTFunctionCallExpression);
    fn visit_assignment_expression(&mut self, expr: &ASTAssignmentExpression) {}

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_variable_expression(&mut self, expr: &ASTVariableExpression);
    fn visit_unary_expression(&mut self, expr: &ASTUnaryExpression);
    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression);
    fn visit_parenthesised_expression(&mut self, expr: &ASTParenthesizedExpression);
    fn visit_binary_operator(&mut self, op: &ASTBinaryOperator);

    fn visit_error(&mut self, span: &TextSpan) {}

    fn visit_integer(&mut self, integer: &i64);
    fn visit_float(&mut self, float: &f64);
}

#[derive(Clone)]
enum ASTStatementKind {
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
    ReturnStatement(ASTReturnStatement),
    CompoundStatement(ASTCompoundStatement),
    FunctionStatement(ASTFunctionStatement),
    ConditionalStatement(ASTConditionalStatement),
}

#[derive(Clone)]
pub struct ASTLetStatement {
    identifier: Token,
    initializer: ASTExpression,
}

#[derive(Clone)]
pub struct ASTReturnStatement {
    expr: ASTExpression,
}
#[derive(Clone)]
pub struct ASTCompoundStatement {
    statements: Vec<ASTStatement>,
}

#[derive(Clone)]
pub struct FunctionArgumentDeclaration {
    identifier: Token,
}

#[derive(Clone)]
pub struct ASTFunctionStatement {
    identifier: Token,
    arguments: Vec<FunctionArgumentDeclaration>,
    body: Box<ASTStatement>,
}

#[derive(Clone)]
pub struct ASTElseStatement {
    else_keyword: Token,
    else_branch: Box<ASTStatement>,
}
#[derive(Clone)]
pub struct ASTConditionalStatement {
    keyword: Token,
    codition: ASTExpression,
    then_branch: Box<ASTStatement>,
    else_branch: Option<ASTElseStatement>,
}

#[derive(Clone)]
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

    fn return_statement(expr: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::ReturnStatement(ASTReturnStatement { expr }),
        }
    }
    fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        Self {
            kind: ASTStatementKind::LetStatement(ASTLetStatement {
                identifier,
                initializer,
            }),
        }
    }

    fn compound(statements: Vec<ASTStatement>) -> Self {
        Self {
            kind: ASTStatementKind::CompoundStatement(ASTCompoundStatement { statements }),
        }
    }

    fn conditional(
        if_token: Token,
        codition: ASTExpression,
        then_branch: ASTStatement,
        else_branch: Option<ASTElseStatement>,
    ) -> Self {
        Self {
            kind: ASTStatementKind::ConditionalStatement(ASTConditionalStatement {
                keyword: if_token,
                codition,
                then_branch: Box::new(then_branch),
                else_branch,
            }),
        }
    }

    fn function(
        identifier: Token,
        arguments: Vec<FunctionArgumentDeclaration>,
        body: ASTStatement,
    ) -> Self {
        Self {
            kind: ASTStatementKind::FunctionStatement(ASTFunctionStatement {
                identifier,
                arguments,
                body: Box::new(body),
            }),
        }
    }
}

#[derive(Clone, PartialEq)]
enum ASTExpressionKind {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    StringLiteral(String),
    Unary(ASTUnaryExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Variable(ASTVariableExpression),
    Assignment(ASTAssignmentExpression),
    FunctionCall(ASTFunctionCallExpression),
    Error(TextSpan),
}

#[derive(Clone, PartialEq)]
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

    fn assignment(token: Token, expr: ASTExpression) -> Self {
        Self {
            kind: ASTExpressionKind::Assignment(ASTAssignmentExpression {
                identifier: token,
                expr: Box::new(expr),
            }),
        }
    }

    fn unary(operator: ASTUnaryOperator, expr: ASTExpression) -> Self {
        Self {
            kind: ASTExpressionKind::Unary(ASTUnaryExpression {
                operator,
                expr: Box::new(expr),
            }),
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

    fn function_call(identifier: Token, arguments: Vec<ASTExpression>) -> Self {
        Self {
            kind: ASTExpressionKind::FunctionCall(ASTFunctionCallExpression {
                identifier,
                arguments,
            }),
        }
    }
}

#[derive(Clone, PartialEq)]
enum ASTUnaryOperatorKind {
    Minus,
    BitwiseNOT,
    LogicNot,
}

#[derive(Clone, PartialEq)]
struct ASTUnaryOperator {
    kind: ASTUnaryOperatorKind,
    token: lexer::Token,
}
#[derive(Clone, PartialEq)]
pub struct ASTUnaryExpression {
    operator: ASTUnaryOperator,
    expr: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    EqualTo,
    NotEqualTo,
    LogicAND,
    LogicOR,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    BitwiseOR,
    BitwiseAND,
    BitwiseXOR,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: lexer::Token,
}

impl ASTBinaryOperator {
    fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus => 5,
            ASTBinaryOperatorKind::Minus => 5,
            ASTBinaryOperatorKind::Multiply => 6,
            ASTBinaryOperatorKind::Divide => 6,
            ASTBinaryOperatorKind::EqualTo => 1,
            ASTBinaryOperatorKind::NotEqualTo => 1,
            ASTBinaryOperatorKind::LogicAND => 1,
            ASTBinaryOperatorKind::LogicOR => 1,
            ASTBinaryOperatorKind::GreaterThan => 1,
            ASTBinaryOperatorKind::GreaterThanOrEqual => 1,
            ASTBinaryOperatorKind::LessThan => 1,
            ASTBinaryOperatorKind::LessThanOrEqual => 1,
            ASTBinaryOperatorKind::BitwiseOR => 1,
            ASTBinaryOperatorKind::BitwiseAND => 1,
            ASTBinaryOperatorKind::BitwiseXOR => 1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ASTBinaryExpression {
    operator: ASTBinaryOperator,
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
}

#[derive(Clone, PartialEq)]
pub struct ASTParenthesizedExpression {
    expr: Box<ASTExpression>,
}

#[derive(Clone, PartialEq)]
pub struct ASTVariableExpression {
    identifier: Token,
}

#[derive(Clone, PartialEq)]
pub struct ASTAssignmentExpression {
    identifier: Token,
    expr: Box<ASTExpression>,
}

impl ASTVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionCallExpression {
    identifier: Token,
    arguments: Vec<ASTExpression>,
}

impl ASTFunctionCallExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}

#[cfg(test)]
mod test {
    use crate::compilation_unit::CompilationUnit;

    use super::lexer::TokenKind;
    use super::ASTVisitor;
    use super::Ast;

    #[derive(Debug, PartialEq)]
    enum TestASTNode {
        Floating(f64),
        Integer(i64),
        Variable(String),
        LetStatement(String),
        ReturnStatement,
        FunctionStatement(Vec<String>),
        BinaryExpr(TokenKind),
        UnaryExpr(TokenKind),
        ParenExpr,
        FunctionCall(String),
    }
    struct ASTVerifier {
        actual: Vec<TestASTNode>,
        expected: Vec<TestASTNode>,
    }

    impl ASTVerifier {
        pub fn new(input: &str, expected_ast: Vec<TestASTNode>) -> Self {
            let compilation_unit = CompilationUnit::compile(input);
            assert!(compilation_unit.is_ok());
            let mut verifier = ASTVerifier {
                actual: Vec::new(),
                expected: expected_ast,
            };

            match compilation_unit {
                Ok(c) => verifier.flatten_ast(&c.ast),
                Err(_) => (),
            };
            verifier
        }

        fn flatten_ast(&mut self, ast: &Ast) {
            ast.visit(&mut *self);
        }

        pub fn verify(&self) {
            assert_eq!(
                self.expected.len(),
                self.actual.len(),
                "Expected {} nodes only has {} ",
                self.expected.len(),
                self.actual.len()
            );

            for (ac, ex) in self.actual.iter().zip(self.expected.iter()) {
                assert_eq!(
                    ac, ex,
                    "Node do not match. Expected {:?} but found {:?}",
                    ex, ac
                )
            }
        }
    }

    impl ASTVisitor for ASTVerifier {
        fn visit_return_statement(&mut self, statement: &super::ASTReturnStatement) {
            self.actual.push(TestASTNode::ReturnStatement);
            self.visit_expression(&statement.expr);
        }

        fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
            self.actual.push(TestASTNode::LetStatement(
                statement.identifier.span.literal.clone(),
            ));
            self.visit_expression(&statement.initializer);
        }

        fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
            let mut args = Vec::new();
            args.push(function.identifier.span.literal.clone());
            for arg in function.arguments.iter() {
                args.push(arg.identifier.span.literal.clone());
            }

            self.actual.push(TestASTNode::FunctionStatement(args));

            if let super::ASTStatementKind::CompoundStatement(statement) = &function.body.kind {
                self.visit_compound_statement(statement);
            }
        }

        fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
            self.actual.push(TestASTNode::FunctionCall(
                expr.identifier.span.literal.clone(),
            ));
            for arg in expr.arguments.iter() {
                self.visit_expression(arg);
            }
        }

        fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
            self.actual
                .push(TestASTNode::Variable(expr.identifier.span.literal.clone()));
        }

        fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
            self.actual
                .push(TestASTNode::BinaryExpr(expr.operator.token.kind.clone()));
            self.visit_expression(&expr.left);
            self.visit_expression(&expr.right);
        }

        fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
            self.actual.push(TestASTNode::ParenExpr);
            self.visit_expression(&expr.expr);
        }

        fn visit_binary_operator(&mut self, op: &super::ASTBinaryOperator) {}

        fn visit_integer(&mut self, integer: &i64) {
            self.actual.push(TestASTNode::Integer(integer.clone()));
        }

        fn visit_float(&mut self, float: &f64) {
            self.actual.push(TestASTNode::Floating(float.clone()));
        }

        fn visit_unary_expression(&mut self, expr: &super::ASTUnaryExpression) {
            self.actual
                .push(TestASTNode::UnaryExpr(expr.operator.token.kind.clone()));
        }
    }

    #[test]
    fn should_parse_let_statement() {
        let input = "let a = 10;";
        let expected_ast = vec![
            TestASTNode::LetStatement("a".to_string()),
            TestASTNode::Integer(10),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }

    #[test]
    fn should_parse_return_statement() {
        let input = "let a = 7;
                           return a + 10;
                           ";
        let expected_ast = vec![
            TestASTNode::LetStatement("a".to_string()),
            TestASTNode::Integer(7),
            TestASTNode::ReturnStatement,
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::Variable("a".to_string()),
            TestASTNode::Integer(10),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }

    #[test]
    fn should_parse_simple_binary_addition_statement() {
        let input = "10 + 3.1415;";
        let expected_ast = vec![
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::Integer(10),
            TestASTNode::Floating(3.1415),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }

    #[test]
    fn should_parse_complex_binary_statement() {
        let input = "let a = (7.2 - 10) / 2 + 3.1415 * 8;";
        let expected_ast = vec![
            TestASTNode::LetStatement("a".to_string()),
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::BinaryExpr(TokenKind::Slash),
            TestASTNode::ParenExpr,
            TestASTNode::BinaryExpr(TokenKind::Minus),
            TestASTNode::Floating(7.2),
            TestASTNode::Integer(10),
            TestASTNode::Integer(2),
            TestASTNode::BinaryExpr(TokenKind::Astrisk),
            TestASTNode::Floating(3.1415),
            TestASTNode::Integer(8),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }

    #[test]
    fn should_parse_function_declaration() {
        let input = "func f(a, b, c) { return a + b + c; }";
        let expected_ast = vec![
            TestASTNode::FunctionStatement(vec![
                "f".to_string(), // function name
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ]),
            TestASTNode::ReturnStatement,
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::Variable("a".to_string()),
            TestASTNode::Variable("b".to_string()),
            TestASTNode::Variable("c".to_string()),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }

    #[test]
    fn should_parse_function_call() {
        let input = "\
        func f(a, b, c) { return a + b + c; }
        f(1, 2, 6)
        ";
        let expected_ast = vec![
            TestASTNode::FunctionStatement(vec![
                "f".to_string(), // function name
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ]),
            TestASTNode::ReturnStatement,
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::BinaryExpr(TokenKind::Plus),
            TestASTNode::Variable("a".to_string()),
            TestASTNode::Variable("b".to_string()),
            TestASTNode::Variable("c".to_string()),
            TestASTNode::FunctionCall("f".to_string()),
            TestASTNode::Integer(1),
            TestASTNode::Integer(2),
            TestASTNode::Integer(6),
        ];

        let verifier = ASTVerifier::new(input, expected_ast);
        verifier.verify();
    }
}
