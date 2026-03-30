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

Decided: yes, lightweight anonymous product types.

**Needs to be specified:**
- Literal syntax: `(1, "hello")`
- Type syntax: `(Int, String)`
- Field access: positional (`.0`, `.1`) or destructuring only?
- Pattern matching on tuples: `match pair { (a, b) => ... }`
- Unit `()` as the zero-element tuple (already in spec, just needs to connect here)

### 3.2 Array / List types

The stdlib mentions `List<T>` but it is never defined as a language construct.

**Needs to be specified:**
- Is `List<T>` a built-in type or a stdlib type?
- Literal syntax: `[1, 2, 3]`
- Indexing syntax: `list[0]`
- Type of index (must be `Int` or `UInt`?)
- Out-of-bounds behavior (panic? `Perhaps<T>`?)

### 3.3 Additional integer type: `UInt`

Decided: `Int` (64-bit signed) + `UInt` (64-bit unsigned).

**Needs to be specified:**
- Literal syntax (suffix? `42u`?)
- Arithmetic between `Int` and `UInt` — is it an error? Requires explicit cast?
- Which contexts require `UInt` (indexing? sizes?)

### 3.4 Type casting / conversion

The `as` keyword is listed in keywords but never defined.

**Needs to be specified:**
- `as` for primitive casts: `let f = x as Float`
- What casts are allowed (Int ↔ Float, Int ↔ UInt, etc.)
- Whether `as` is the only mechanism or if `From`/`Into` traits are also added

---

## 4. Compound Assignment Operators

Decided: yes.

**Needs to be specified in the spec:**
- `+=`, `-=`, `*=`, `/=`, `%=`
- Add to the operators table and grammar

Small spec change, but should be done before the grammar is considered stable.

---

## 5. Loop Control

The spec defines `while` and `for` but has no way to break out of a loop early or skip an iteration.

**Needs to be specified:**
- `break` — exit the current loop
- `continue` — skip to the next iteration
- `loop` — infinite loop construct (replaces `while (true)`)
- Whether `break` can carry a value (Rust allows `break value` from `loop`)

---

## 6. Associated Functions and Constructors

The spec only covers methods with `self`. There is no constructor pattern.

**Needs to be specified:**
- Associated functions (no `self` parameter) in `impl` blocks
- The convention for constructors: `Type::new(...)` as an associated function
- Whether there is any special constructor syntax or it is purely convention

**Example gap:**
```yolo
impl Point {
    // This is currently unspecified — how do you construct without a receiver?
    fun new(x: Float, y: Float) -> Point {
        return Point { x: x, y: y };
    }
}
```

---

## 7. Mutable Self in Methods

Decided: mutating methods must declare `mut self`.

**Needs to be specified:**
- Syntax: `fun push(mut self, value: T)`
- Whether `mut self` consumes and returns self, or mutates in place
- How this interacts with reference counting (since the language has no ownership)

---

## 8. Operator Overloading

Decided: yes, via traits (Rust-style).

**Needs to be specified:**
- The set of built-in operator traits: `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Not`, `Eq`, `Ord`, etc.
- Trait method signatures for each
- Which operators map to which traits
- Whether comparison traits (`Eq`, `Ord`) are derivable automatically for structs/enums

---

## 9. String Interpolation

The spec only supports string concatenation with `+`, which is verbose.

**Needs to be designed:**
- Syntax: template literals `` `hello ${name}` ``? Format strings `"hello {name}"`? Something else?
- Whether any value can be interpolated or only types implementing a `Display`-like trait
- Relationship to the `print` builtin

---

## 10. `?` Operator — Full Specification

The spec sketches `?` for `Result` but leaves details open.

**Decided:** `?` works only on `Result<T, E>`, not on `Perhaps<T>`.

**Needs to be specified:**
- The full desugaring of `expr?`
- Error type compatibility — what if the function returns `Result<T, E1>` but `?` is applied to `Result<U, E2>`? Is there an implicit conversion, or must `E1 == E2`?
- Whether a `From` trait is used for error coercion (as in Rust)

---

## 11. Trait Objects / Dynamic Dispatch

The spec currently only supports static dispatch via generics. There is no way to store a heterogeneous collection of values that implement a trait.

**Needs to be designed:**
- Syntax: `dyn Trait`? Something else?
- Whether this is in scope at all, or explicitly deferred
- Fat pointer semantics vs. some other model

---

## 12. Derived / Auto-implemented Traits

For types like `Eq`, `Ord`, `Display`, manually writing `impl` blocks is tedious.

**Needs to be designed:**
- Is there a `derive` mechanism or annotation?
- Which traits can be auto-derived?
- Syntax: `#[derive(Eq, Ord)]`? Something less Rust-like?

---

## 13. Closures — Type Signatures

The spec describes closures but does not specify their type when used as function parameters or return values.

**Needs to be specified:**
- How to annotate a parameter that accepts a closure: `fun apply(f: fun(Int) -> Int, x: Int) -> Int`?
- Whether there are separate function pointer and closure types
- How closures interact with `mut` captures in a typed context

---

## 14. Panic and Program Termination

The spec mentions `.yolo()` panics but does not define what a panic is.

**Needs to be specified:**
- What happens on panic (unwind? abort? configurable?)
- Whether panics can be caught
- Any other sources of panics (out-of-bounds, integer overflow, etc.)

---

## Summary Table

| #   | Topic                         | Status         | Blocks                        |
| --- | ----------------------------- | -------------- | ----------------------------- |
| 1   | Module system                 | ❌ Not started  | stdlib, multi-file code       |
| 2   | Visibility (`pub`)            | ❌ Not started  | modules, structs              |
| 3.1 | Tuples                        | ❌ Not started  | pattern matching completeness |
| 3.2 | Array / List                  | ❌ Not started  | stdlib, for-in loops          |
| 3.3 | `UInt` type                   | ❌ Not started  | indexing, casting             |
| 3.4 | Type casting (`as`)           | ❌ Not started  | numeric operations            |
| 4   | Compound assignment           | ❌ Not started  | grammar stability             |
| 5   | `break` / `continue` / `loop` | ❌ Not started  | —                             |
| 6   | Associated functions          | ❌ Not started  | constructors, stdlib          |
| 7   | `mut self` semantics          | ❌ Not started  | mutable methods               |
| 8   | Operator overloading traits   | ❌ Not started  | custom types                  |
| 9   | String interpolation          | ❌ Not designed | ergonomics                    |
| 10  | `?` full specification        | ⚠️ Partial     | error propagation             |
| 11  | Trait objects / `dyn`         | ❌ Not designed | dynamic dispatch              |
| 12  | Derived traits                | ❌ Not designed | ergonomics                    |
| 13  | Closure type signatures       | ❌ Not started  | higher-order functions        |
| 14  | Panic semantics               | ❌ Not started  | `.yolo()`, runtime errors     |
