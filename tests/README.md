# Yolang v0.1 Test Files

Each file is a self-contained Yolang program that exercises a specific part of the v0.1 feature set. Every file has a `main` function as the entry point. Expected output is annotated inline as comments.

| File | Feature area |
|------|-------------|
| `01_literals_and_variables.yolo` | Integer/float/string/bool/nope literals, `let`, `mut`, type inference, shadowing, compound assignment |
| `02_control_flow.yolo` | `if`/`else` (statement + expression), `while`, C-style `for`, `for-in`, `loop`, `break`, `break expr`, `continue` |
| `03_functions_and_closures.yolo` | Named functions, first-class functions, closures, captures, `mut` captures, generic functions, higher-order functions, closure type signatures |
| `04_structs_and_impl.yolo` | Struct definition, instantiation, field access, methods, `mut self`, associated functions (constructors), generic structs |
| `05_enums_and_match.yolo` | Unit and data-carrying enum variants, `impl` on enums, `match` as statement and expression, destructuring, guards, `Perhaps<T>`, `nope`, `Result<T,E>`, `.yolo()` |
| `06_traits.yolo` | Trait definition, `impl Trait for Type`, default methods, `Self` type, trait bounds in generic functions |
| `07_arrays_and_tuples.yolo` | `T[]` / `Array<T>` literals, indexing, `array_push`, `array_len`, `for-in`, tuple literals, positional access, tuple destructuring in `match` |
| `08_error_handling.yolo` | `Result<T,E>`, `?` operator, chained `?`, `Perhaps<T>`, `.yolo()`, `match` on results |
| `09_casting_and_generics.yolo` | `as` (Int→Float, Float→Int), generic structs, generic functions, `where` clauses, inline trait bounds |
| `10_comprehensive.yolo` | Integration test: generic Stack, enum tokens, task list with priorities — exercises most features together |
