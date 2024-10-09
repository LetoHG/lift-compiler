use super::sourcetext;
use super::Diagnostic;

use termion::color::Fg;
use termion::color::Red;
use termion::color::Reset;

pub struct DiagnosticsPrinter<'a> {
    source_text: &'a sourcetext::SourceText,
    // diagnostics: &'a [Diagnostic],
}

impl<'a> DiagnosticsPrinter<'a> {
    pub fn new(
        source_text: &'a sourcetext::SourceText, /* diagnostics: &'a [Diagnostic] */
    ) -> Self {
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
        let (line, col) = self.source_text.get_location(diagnostic.span.start);
        let whitespace = " ".repeat(col);
        format!(
            "{}{line}{}{whitespace}^\n{whitespace}│\n{whitespace}└─{}",
            Fg(Red),
            Fg(Reset),
            diagnostic.message
        )
        .to_string()
    }
}
