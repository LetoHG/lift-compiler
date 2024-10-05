use cli_calculator::evaluate;
use std::fs;

fn main() {
    let content = fs::read_to_string("math.txt").unwrap();
    evaluate(content.as_str());
}
