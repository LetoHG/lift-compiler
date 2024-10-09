mod ast;
mod diagnostics;

use ast::lexer::Token;
use diagnostics::sourcetext::SourceText;
use diagnostics::DiagnosticsColletionCell;
use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    let source_text = SourceText::from_file("README.md");

    let content = fs::read_to_string("math.txt").unwrap();

    let mut lexer = ast::lexer::Lexer::new(content.clone());
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    let diagnostics_colletion: DiagnosticsColletionCell =
        Rc::new(RefCell::new(DiagnosticsColletion::new()));

    // let printer = diagnostics::printer::DiagnosticsPrinter::new(&source_text);
    // println!(
    //     "printer: {}",
    //     printer.stringify_diagnostic(
    //         diagnostics_colletion
    //             .borrow_mut()
    //             .diagnostics
    //             .first()
    //             .unwrap()
    //     )
    // );

    let mut ast = ast::Ast::new();
    let mut parser = ast::parser::Parser::new(tokens, Rc::clone(&diagnostics_colletion));
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();
}
