use std::{cell::RefCell, rc::Rc};

use crate::ast::lexer::{Token, TokenKind};

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    message: String,
    kind: DiagnosticKind,
    token: Token,
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
    diagnostics: Vec<Diagnostic>,
}

pub type DiagnosticsColletionCell = Rc<RefCell<DiagnosticsColletion>>;

impl DiagnosticsColletion {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn report_error(&mut self, message: String, token: Token) {
        self.diagnostics
            .push(Diagnostic::new(message, DiagnosticKind::Error, token));
    }

    pub fn report_warning(&mut self, message: String, token: Token) {
        self.diagnostics
            .push(Diagnostic::new(message, DiagnosticKind::Warning, token));
    }

    pub fn report_unexpected_token(&mut self, expected_tokenkind: TokenKind, found_token: Token) {
        self.report_error(
            format!(
                "Expected <{}> but found <{}>",
                expected_tokenkind, found_token.kind
            ),
            found_token,
        );
    }
}
