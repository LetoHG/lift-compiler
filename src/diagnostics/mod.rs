use std::{cell::RefCell, rc::Rc};

use crate::ast::lexer::{Token, TokenKind};

pub mod printer;
pub mod sourcetext;
pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    pub(crate) message: String,
    pub(crate) kind: DiagnosticKind,
    pub(crate) token: Token,
}

impl Diagnostic {
    pub fn new(message: String, kind: DiagnosticKind, token: Token) -> Self {
        Self {
            message,
            kind,
            token,
        }
    }
}

pub struct DiagnosticsColletion {
    pub diagnostics: Vec<Diagnostic>,
}

pub type DiagnosticsColletionCell = Rc<RefCell<DiagnosticsColletion>>;

impl DiagnosticsColletion {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn report_error(&mut self, message: String, token: &Token) {
        self.diagnostics.push(Diagnostic::new(
            message,
            DiagnosticKind::Error,
            token.clone(),
        ));
    }

    pub fn report_warning(&mut self, message: String, token: &Token) {
        self.diagnostics.push(Diagnostic::new(
            message,
            DiagnosticKind::Warning,
            token.clone(),
        ));
    }

    pub fn report_unexpected_token(&mut self, expected_tokenkind: &TokenKind, found_token: &Token) {
        self.report_error(
            format!(
                "Expected <{}>, but found <{}>",
                expected_tokenkind, found_token.kind
            ),
            found_token,
        );
    }
    pub fn report_expected_expression(&mut self, found_token: &Token) {
        self.report_error(
            format!("Expected expression, but found <{}>", found_token.kind),
            found_token,
        );
    }
}
