use crate::diagnostics::sourcetext::SourceText;
use crate::diagnostics::Diagnostic;

use super::sourcetext;

pub struct DiagnosticsPrinter<'a> {
    source_text: &'a SourceText,
    // diagnostics: &'a [Diagnostic],
}

impl<'a> DiagnosticsPrinter<'a> {
    pub fn new(source_text: &'a SourceText /* diagnostics: &'a [Diagnostic] */) -> Self {
        Self {
            source_text,
            // diagnostics,
        }
    }

    // let x = 5;
    //     ^
    //     │
    //     └─
    //
    //
    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let (line, col) = self.source_text.get_location(diagnostic.token.span.start);
        let whitespace = " ".repeat(col);
        format!(
            "{line}{whitespace}^\n{whitespace}│\n{whitespace}└─{}",
            diagnostic.message
        )
        .to_string()
    }
}
