use cli_calculator::ast::lexer::{Token, TokenKind};
use cli_calculator::evaluate;
use std::fs;

fn main() {
    let content = fs::read_to_string("math.txt").unwrap();
    let res = evaluate(content.as_str()).unwrap_or(0.0);
    println!("{} = {}", content.strip_suffix("\n").unwrap(), res);

    let mut lexer = cli_calculator::ast::lexer::Lexer::new(content.clone());
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let upn = cli_calculator::Upn::create(
        tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .map(|token| token.clone())
            .collect(),
    );

    let res = upn.solve().unwrap();
    println!(
        "New Upn solver: {} = {}",
        content.strip_suffix("\n").unwrap(),
        res
    );

    let mut ast = cli_calculator::ast::Ast::new();
    let mut parser = cli_calculator::ast::parser::Parser::new(tokens);
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();
}
