mod ast;
mod compilation_unit;
mod diagnostics;
mod source_text;

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
    let compilation_unit = CompilationUnit::compile(input)?;
    compilation_unit.run();
    Ok(())
}
