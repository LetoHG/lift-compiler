use cli_calculator::evaluate;
use std::fs;

fn main() {
    let content = fs::read_to_string("math.txt").unwrap();
    let res = evaluate(content.as_str()).unwrap_or(0.0);
    println!("{} = {}", content.strip_suffix("\n").unwrap(), res);
}
