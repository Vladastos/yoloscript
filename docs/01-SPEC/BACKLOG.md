# Yoloscript Spec Backlog

Open design questions and deferred features. When an item is resolved, it is removed from this file and written into `Language Spec.md`. See `PROCESS.md` for the full workflow.

**Statuses:** `open` — needs design · `deferred` — consciously excluded for now

---

## Type system

| Item | Status | Notes |
|------|--------|-------|
| Let-polymorphism (implicit generalization) | ~~`open`~~ **DECIDED** | Let-bound closures can be used polymorphically without explicit `<T>` params via Hindley-Milner style generalization (on binding) + instantiation (on use). Implement in Epic 003 task 0010. |
| `UInt` type (64-bit unsigned) | `deferred` | Adds casting/arithmetic complexity; `Int` is sufficient for v0.1. Literal suffix TBD (`42u`?). Will need casting rules with `Int`. |
| `Int` ↔ `UInt` casting via `as` | `deferred` | Blocked on `UInt` being added. |
| `TryFrom` / `Into` traits | `deferred` | Complement to `From`/`as`; useful for fallible casts. Deferred until needed. |
| Trait objects (`dyn Trait`) | `deferred` | Fat-pointer semantics not yet designed. Syntax TBD (`dyn Trait`?). |

---

## Standard library

| Item | Status | Notes |
|------|--------|-------|
| `List<T>` type | `deferred` | Higher-level sequence type on top of `Array<T>`. Deferred until a module system exists or stdlib is fleshed out. |
| `math` module | `open` | `floor`, `ceil`, `abs`, `sqrt`, `pow`, `min`, `max` |
| `string` module | `open` | `split`, `trim`, `contains`, `to_upper`, `to_lower` |
| `io` module | `open` | `read_line`, `read_file`, `write_file` |

---

## Language features

| Item | Status | Notes |
|------|--------|-------|
| Module system / `use` | `deferred` | File-to-module mapping not designed. Blocks stdlib, multi-file programs, and visibility. |
| Visibility (`pub`) | `deferred` | Depends on the module system. Default: private; `pub` to export. |
| `pub use` re-exports | `deferred` | Depends on module system. |
| String interpolation | `deferred` | Syntax not decided. Options: `` `hello ${name}` ``, `"hello {name}"`, or other. Must decide whether any value is interpolatable or only `Display`-implementing types. |
| Operator overloading traits | `deferred` | Via traits (`Add`, `Sub`, `Mul`, etc.), Rust-style. Deferred because it can be added without breaking existing code. Needs: trait definitions, method signatures, operator-to-trait mapping. |
| Derived / auto-impl traits | `deferred` | Syntax not designed. `#[derive(...)]` is too Rust-like. Needed for `Eq`, `Ord`, `Display`, etc. |
| `?` error type coercion | ~~`deferred`~~ **SPEC** | Specified in §5.4: `E2: From<E1>` semantics with v0.1 note. Implementation deferred to Epic 004 task 0003. |
| Integer overflow behaviour | `open` | Currently wrapping. Should be: panic in debug, wrapping in release? Configurable? |
| Panic recovery | `deferred` | No way to catch panics in v0.1. If catch semantics are added, needs careful design. |

---

## Ergonomics

| Item | Status | Notes |
|------|--------|-------|
| `UInt` as array index type | `deferred` | Blocked on `UInt`. Currently `Int` is used for indexing. |
| `List<T>` higher-level API | `deferred` | `map`, `filter`, `fold`, `pop`, `push`, etc. Blocked on `List<T>`. |
