mod ast;

use cli_calculator::evaluate;
use std::fs;

fn main() {
    let content = fs::read_to_string("math.txt").unwrap();
    let res = evaluate(content.as_str()).unwrap_or(0.0);
    println!("{} = {}", content.strip_suffix("\n").unwrap(), res);

    let mut lexer = ast::lexer::Lexer::new(content);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    // println!("{:?}", tokens);

    let mut ast = ast::Ast::new();
    let mut parser = ast::parser::Parser::new(tokens);
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();
}
