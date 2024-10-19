mod ast;
mod compilation_unit;
mod diagnostics;
mod source_text;

use std::fs;

use crate::compilation_unit::CompilationUnit;

fn main() -> Result<(), ()> {
    // func a() { return 10; }
    let input = "\
func a(arg1, arg2) {
  let c = arg2 / 071
  return arg1 * c
}

return a(0x02, 7.67)
";

    let content = fs::read_to_string("math.txt").unwrap();

    let compilation_unit = CompilationUnit::compile(content.as_str())?;
    compilation_unit.run();
    Ok(())
}
