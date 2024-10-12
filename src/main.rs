mod ast;
mod diagnostics;

use ast::lexer::Token;
use ast::printer::ASTHiglightPrinter;
use ast::solver::ASTSolver;
use ast::symbol_checker;
use diagnostics::printer::DiagnosticsPrinter;
use diagnostics::sourcetext::SourceText;
use diagnostics::{DiagnosticsColletion, DiagnosticsColletionCell};
use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    // func a() { return 10; }
    let input = "
func a() {
    let aligator = 10;
    let elephant = 2.15;
    let b = 7 - elephant + aligator;
    let crocodile = aligator + 3.1415 / (2 * b);
    return aligator;
}
return a();
";

    // let source_text = SourceText::from_file("math.txt");
    let source_text = SourceText::new(input.to_string());

    // let content = fs::read_to_string("math.txt").unwrap();

    // let mut equation = String::new();
    // std::io::stdin().read_line(&mut equation).unwrap();

    let mut lexer = ast::lexer::Lexer::new(input.to_string());
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let diagnostics_colletion: DiagnosticsColletionCell =
        Rc::new(RefCell::new(DiagnosticsColletion::new()));

    let mut ast = ast::Ast::new();
    let mut parser = ast::parser::Parser::new(tokens, Rc::clone(&diagnostics_colletion));
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();

    let mut highlight_printer = ASTHiglightPrinter::new();
    ast.visit(&mut highlight_printer);
    highlight_printer.print_result();

    println!(
        "Synatx Errors: {}",
        diagnostics_colletion.borrow_mut().diagnostics.len()
    );
    print_diagstics(&source_text, &diagnostics_colletion);
    diagnostics_colletion.borrow_mut().clear();

    let mut symbol_checker = symbol_checker::SymbolChecker::new(Rc::clone(&diagnostics_colletion));
    ast.visit(&mut symbol_checker);
    println!(
        "Indentifier Errors: {}",
        diagnostics_colletion.borrow_mut().diagnostics.len()
    );
    print_diagstics(&source_text, &diagnostics_colletion);

    if diagnostics_colletion.borrow().diagnostics.len() > 0 {
        return;
    }

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
