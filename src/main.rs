mod ast;
mod compilation_unit;
mod diagnostics;
mod source_text;

use crate::compilation_unit::CompilationUnit;

fn main() -> Result<(), ()> {
    // func a() { return 10; }
    let input = "
func a() {
    let elephant = 2.15;
    let aligator = (10 + 2) * elephant + 4;
    let b = 7 - elephant + aligator;
    let crocodile = aligator + 3.1415 / (2 * b);
    return aligator;
}
return a();
";

    let compilation_unit = CompilationUnit::compile(input)?;
    compilation_unit.run();
    Ok(())
}
