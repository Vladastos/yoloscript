# Language Spec Roadmap

This document tracks what is missing from or underspecified in the Language Spec, and the order in which those gaps should be designed and written up.

Items are grouped by theme. Within each group they are loosely ordered by how much other things depend on them — foundational decisions come first.

---

## 1. Module System

Nothing in the current spec addresses how code is organized across files.

**Needs to be designed:**
- File-to-module mapping (one file = one module? explicit `mod` declarations?)
- The `use` keyword — path syntax for importing names (`use math::sqrt`, `use collections::List`)
- Visibility — `pub` to export, private by default (decided)
- Re-exports (`pub use`)
- The standard library module structure

**Depends on:** visibility model (decided: private by default, `pub` to export)

**Blocks:** standard library design, any multi-file example code in the spec

---

## 2. Visibility

Closely tied to modules but also affects structs and traits independently.

**Needs to be specified:**
- Default visibility for struct fields (private? public?)
- Default visibility for `impl` methods
- Default visibility for top-level functions and type definitions
- The `pub` keyword placement rules (`pub struct`, `pub fun`, `pub field_name: Type`)

**Decided:** private by default, `pub` to opt in

---

## 3. Type System Gaps

### 3.1 Tuples

✅ **Resolved for v0.1** (see decision 0001).

- Literal syntax: `(1, "hello")`
- Type syntax: `(Int, String)`
- Positional field access: `.0`, `.1`, ...
- Pattern matching via destructuring: `match pair { (a, b) => ... }`
- `()` is the zero-element tuple (unit type), already in spec

### 3.2 Array / List types

✅ **Resolved for v0.1** (see decision 0001).

- `Array<T>` / `T[]` is the built-in ordered sequence type.
- Literal syntax: `[1, 2, 3]`
- Indexing: `arr[i]` where `i: Int`; out-of-bounds panics.
- `List<T>` is a stdlib type built on `Array<T>`, pre-imported in v0.1.

**Still open for later versions:**
- Full stdlib `List<T>` API (`map`, `filter`, `fold`, etc.)
- Whether `UInt` becomes the required index type once added

### 3.3 Additional integer type: `UInt`

Decided: `Int` (64-bit signed) + `UInt` (64-bit unsigned).

**Needs to be specified:**
- Literal syntax (suffix? `42u`?)
- Arithmetic between `Int` and `UInt` — is it an error? Requires explicit cast?
- Which contexts require `UInt` (indexing? sizes?)

### 3.4 Type casting / conversion

✅ **Resolved for v0.1** (see decision 0001).

- `as` desugars to `From` — casts are infallible and return the target type directly.
- Allowed in v0.1: `Int` ↔ `Float`.
- User types can become castable by implementing `From<SourceType>`.

**Still open for later versions:**
- `Int` ↔ `UInt` casting once `UInt` is added
- Whether `TryFrom` / `Into` traits are provided for fallible or reverse casts

---

## 4. Compound Assignment Operators

✅ **Resolved for v0.1** (see decision 0001).

- `+=`, `-=`, `*=`, `/=`, `%=` are all included.
- Grammar and operators table in the spec need updating.

---

## 5. Loop Control

✅ **Resolved for v0.1** (see decision 0001).

- `break` — exits the innermost loop
- `continue` — skips to the next iteration
- `loop { }` — infinite loop construct
- `break expr` — carries a value out of a `loop` expression (e.g. `let x = loop { break 42; };`)

---

## 6. Associated Functions and Constructors

✅ **Resolved for v0.1** (see decision 0001).

- `impl` blocks may contain functions with no `self` parameter (associated functions).
- Constructor convention: `Type::new(...)` as an associated function — no special syntax.

```yolo
impl Point {
    fun new(x: Float, y: Float) -> Point {
        return Point { x: x, y: y };
    }
}
```

---

## 7. Mutable Self in Methods

✅ **Resolved for v0.1** (see decision 0001).

- Syntax: `fun push(mut self, value: T)`
- `mut self` mutates in place (no consume-and-return); consistent with RC memory model.
- The receiver variable inside the method body is mutable.

---

## 8. Operator Overloading

⏳ **Deferred — post v0.1** (see decision 0001).

Decided: yes, via traits (Rust-style). Design is not yet settled enough to implement.

**Needs to be specified:**
- The set of built-in operator traits: `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Not`, `Eq`, `Ord`, etc.
- Trait method signatures for each
- Which operators map to which traits
- Whether comparison traits (`Eq`, `Ord`) are derivable automatically for structs/enums

---

## 9. String Interpolation

⏳ **Deferred — post v0.1** (see decision 0001).

`int_to_string` + `+` concatenation is sufficient for v0.1. Syntax not yet decided.

**Needs to be designed:**
- Syntax: template literals `` `hello ${name}` ``? Format strings `"hello {name}"`? Something else?
- Whether any value can be interpolated or only types implementing a `Display`-like trait
- Relationship to the `print` builtin

---

## 10. `?` Operator — Full Specification

✅ **Resolved for v0.1** (see decision 0001).

- `?` works only on `Result<T, E>`, not on `Perhaps<T>`.
- Desugaring: if `Err(e)`, return early with `Err(e)`; if `Ok(v)`, evaluate to `v`.
- Error types must match exactly in v0.1 — no implicit coercion via `From`.

**Still open for later versions:**
- `From`-based error coercion (so `?` can convert between compatible error types)

---

## 11. Trait Objects / Dynamic Dispatch

⏳ **Deferred — post v0.1** (see decision 0001).

Static dispatch via generics is sufficient for v0.1. Fat-pointer semantics not yet designed.

**Needs to be designed:**
- Syntax: `dyn Trait`? Something else?
- Fat pointer semantics vs. some other model

---

## 12. Derived / Auto-implemented Traits

⏳ **Deferred — post v0.1** (see decision 0001).

Syntax not yet designed; `#[derive(...)]` is too Rust-like for Yolang's aesthetic.

**Needs to be designed:**
- Is there a `derive` mechanism or annotation?
- Which traits can be auto-derived?
- Syntax that fits Yolang's style

---

## 13. Closures — Type Signatures

✅ **Resolved for v0.1** (see decision 0001).

- Closure type syntax reuses `fun` notation: `fun(Int) -> Int`
- No separate function pointer vs. closure types in v0.1
- Example: `fun apply(f: fun(Int) -> Int, x: Int) -> Int`

**Still open for later versions:**
- How `mut` captures interact with the type system in more complex scenarios

---

## 14. Panic and Program Termination

✅ **Resolved for v0.1** (see decision 0001).

- A panic is a hard, unrecoverable runtime error: prints a message and exits with a non-zero status.
- Panics cannot be caught in v0.1.
- Sources: `.yolo()` on `nope` or `Err`, out-of-bounds array access, integer division by zero.

---

## Summary Table

| #   | Topic                         | Status              | Blocks                        |
| --- | ----------------------------- | ------------------- | ----------------------------- |
| 1   | Module system                 | ⏳ Deferred (v0.2+) | stdlib, multi-file code       |
| 2   | Visibility (`pub`)            | ⏳ Deferred (v0.2+) | modules, structs              |
| 3.1 | Tuples                        | ✅ v0.1             | —                             |
| 3.2 | Array / List                  | ✅ v0.1             | —                             |
| 3.3 | `UInt` type                   | ⏳ Deferred (v0.2+) | indexing, casting             |
| 3.4 | Type casting (`as`)           | ✅ v0.1             | —                             |
| 4   | Compound assignment           | ✅ v0.1             | —                             |
| 5   | `break` / `continue` / `loop` | ✅ v0.1             | —                             |
| 6   | Associated functions          | ✅ v0.1             | —                             |
| 7   | `mut self` semantics          | ✅ v0.1             | —                             |
| 8   | Operator overloading traits   | ⏳ Deferred (v0.2+) | custom types                  |
| 9   | String interpolation          | ⏳ Deferred (v0.2+) | ergonomics                    |
| 10  | `?` full specification        | ✅ v0.1             | —                             |
| 11  | Trait objects / `dyn`         | ⏳ Deferred (v0.2+) | dynamic dispatch              |
| 12  | Derived traits                | ⏳ Deferred (v0.2+) | ergonomics                    |
| 13  | Closure type signatures       | ✅ v0.1             | —                             |
| 14  | Panic semantics               | ✅ v0.1             | —                             |
