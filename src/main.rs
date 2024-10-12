mod ast;
mod diagnostics;

use ast::lexer::Token;
use ast::printer::ASTHiglightPrinter;
use ast::solver::ASTSolver;
use diagnostics::printer::DiagnosticsPrinter;
use diagnostics::sourcetext::{self, SourceText};
use diagnostics::{DiagnosticsColletion, DiagnosticsColletionCell};
use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    let input = "
        let a = 10;
        let b = 7 + a;
        let c = 3.1415 / 2;
    "
    .to_string();

    // let source_text = SourceText::from_file("math.txt");
    let source_text = SourceText::new(input.clone());

    // let content = fs::read_to_string("math.txt").unwrap();

    // let mut equation = String::new();
    // std::io::stdin().read_line(&mut equation).unwrap();

    let mut lexer = ast::lexer::Lexer::new(input.clone());
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let diagnostics_colletion: DiagnosticsColletionCell =
        Rc::new(RefCell::new(DiagnosticsColletion::new()));

    let mut ast = ast::Ast::new();
    let mut parser = ast::parser::Parser::new(tokens, Rc::clone(&diagnostics_colletion));
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();

    print_diagstics(&source_text, &diagnostics_colletion);

    let mut highlight_printer = ASTHiglightPrinter::new();
    ast.visit(&mut highlight_printer);
    highlight_printer.print_result();

    let mut solver = ASTSolver::new();
    ast.visit(&mut solver);
    solver.print_result();
}

fn print_diagstics(source_text: &SourceText, diagnostics_colletion: &DiagnosticsColletionCell) {
    let diagnostics_messages = &diagnostics_colletion.borrow().diagnostics;
    if diagnostics_messages.len() > 0 {
        let diagnostics_printer = DiagnosticsPrinter::new(&source_text, &diagnostics_messages);
        diagnostics_printer.print();
    }
}
