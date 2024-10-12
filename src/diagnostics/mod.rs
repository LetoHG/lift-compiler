pub mod printer;
pub mod sourcetext;

use crate::ast::lexer::{TextSpan, Token, TokenKind};
use std::{cell::RefCell, rc::Rc};

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    pub(crate) message: String,
    pub(crate) kind: DiagnosticKind,
    pub(crate) span: TextSpan,
}

impl Diagnostic {
    pub fn new(message: String, kind: DiagnosticKind, span: TextSpan) -> Self {
        Self {
            message,
            kind,
            span,
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

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn report_error(&mut self, message: String, span: TextSpan) {
        self.diagnostics
            .push(Diagnostic::new(message, DiagnosticKind::Error, span));
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        self.diagnostics
            .push(Diagnostic::new(message, DiagnosticKind::Warning, span));
    }

    pub fn report_unexpected_token(&mut self, expected_tokenkind: &TokenKind, found_token: &Token) {
        self.report_error(
            format!(
                "Expected <{}>, but found <{}>",
                expected_tokenkind, found_token.kind
            ),
            found_token.span.clone(),
        );
    }
    pub fn report_expected_expression(&mut self, found_token: &Token) {
        self.report_error(
            format!("Expected expression, but found <{}>", found_token.kind),
            found_token.span.clone(),
        );
    }

    pub fn report_undefined_variable(&mut self, span: TextSpan) {
        self.report_error(format!("Not found in this scope"), span);
    }
}
