# lift-compiler

When I decided to learn Rust, I initially struggled to come up with project ideas and find the motivation to start coding. Coming from a C++ background, I thought it would be interesting to try implementing a compiler in Rust. After all, compilers are the tools that have enabled my journey in programming, and creating one could deepen my understanding of both languages.

## Syntax

Input:

```
func a(arg1, arg2) {
  let c = arg2 / 1.5
  return arg1 * c
}

return a(1, 7.67)
```

Output:

```
Highlighted Source:
func a(arg1arg2) {
  let c = arg2 / 1.5
  return arg1 * c
}
return a(2,  7.67)
Synatx Errors: 0
Indentifier Errors: 0
Solver result: 10.226666666666667
```
