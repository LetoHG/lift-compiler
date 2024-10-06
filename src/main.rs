mod ast;

use cli_calculator::evaluate;
use std::fs;

fn main() {
    let content = fs::read_to_string("math.txt").unwrap();
    println!("{} = {}", content.strip_suffix("\n").unwrap(), res);
    evaluate(content.as_str());

    let mut lexer = ast::lexer::Lexer::new(content);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);
}
