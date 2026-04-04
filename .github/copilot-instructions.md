# Yolang Agent Instructions

You are helping develop **Yolang**, a strongly typed, compiled programming language with a Rust-inspired type system. This document makes you immediately productive in the codebase.

## Project Overview

**Goal:** Build a tree-walk interpreter that validates the v0.1 language specification. Performance is secondary; correctness and spec fidelity are paramount.

**Key properties:**
- Statically typed with local type inference
- No classes; types defined via structs, enums, traits
- Algebraic data types with exhaustive pattern matching
- Explicit nullability (`Perhaps<T>`), explicit errors (`Result<T,E>`)
- v0.1 feature set is complete and frozen (see `specs/Language Spec.md`)

**Related docs:**
- [Language Spec](docs/Yolang/spec/Language%20Spec.md) — authoritative feature definitions
- [Backlog](docs/Yolang/spec/Backlog.md) — open design questions and deferred features
- [Design Decisions](docs/Yolang/decisions/) — rationale for architectural choices
- [Documentation Process](docs/Yolang/PROCESS.md) — how specs, backlogs, decisions interact
- [Interpreter Design](docs/Yolang/interpreter/DESIGN.md) — full pipeline architecture

## Build & Test

All commands run from `tree-walk-interpreter/`:

```bash
# Build the interpreter
cargo build

# Run a .yolo script directly
cargo run -- tests/01_literals_and_variables.yolo

# Run all v0.1 tests (validate spec implementation)
cargo run -- tests/10_comprehensive.yolo  # etc.
```

**Key test files:** `tests/0N_*.yolo` validate each feature area. They are self-contained, include inline expected output as comments, and map to v0.1 feature set. See [test README](tests/README.md) for the full feature mapping.

## Architecture: The Interpreter Pipeline

```
source.yolo
    ↓
[Parser]      pest PEG grammar → concrete syntax tree (CST)
    ↓
[AST Builder] CST → untyped abstract syntax tree
    ↓
[Type Checker] untyped AST → typed AST  (type errors caught here)
    ↓
[Evaluator]   typed AST → program output  (tree-walking execution)
```

Each stage is a **separate Rust module**. They communicate through well-defined data structures. **No stage skips another.**

### Source Code Layout

```
src/
├── main.rs            – CLI: reads .yolo file, runs pipeline end-to-end
├── grammar.pest       – Complete v0.1 grammar (pest PEG syntax)
├── parser/mod.rs      – Drives pest, builds untyped AST from CST
├── ast/mod.rs         – Untyped AST node definitions
├── types/mod.rs       – Type representation (used by type checker, evaluator)
├── typechecker/mod.rs – Type inference, checking, monomorphisation
├── typed_ast/mod.rs   – Typed AST (AST nodes annotated with resolved types)
├── evaluator/mod.rs   – Tree-walking evaluation, environment, runtime values
└── error/mod.rs       – Unified error type covering all stages
```

**Key file responsibilities:**
- **grammar.pest**: Source of truth for syntax. When adding a feature, define syntax here first.
- **parser/mod.rs**: Builds the untyped AST. Errors here are syntax errors.
- **ast/mod.rs**: Node types. Mirrors grammar structure. Uses `Span` for error location.
- **typechecker/mod.rs**: Implements untyped→typed conversion. Type errors, generics, monomorphisation all happen here.
- **typed_ast/mod.rs**: Mirrors untyped AST but each node carries its resolved type.
- **evaluator/mod.rs**: Tree-walking eval. Runtime values, environments, function calls.

## Documentation Structure

Yolang uses a strict, non-overlapping documentation system (see [PROCESS.md](docs/Yolang/PROCESS.md)):

### 1. `Language Spec.md` — The Source of Truth
- **Only place** where language features are authoritatively described
- Answers: "How does the language work?"
- No rationale, no history, no open questions — only feature definitions
- When a feature is complete, written into spec. Proof it's done.

### 2. `Backlog.md` — What's Not Done Yet
- Tracks every open design question and deferred feature
- Answers: "What still needs to be designed or implemented?"
- Each item marked: `open` (needs design), `deferred` (consciously excluded for v0.1), `in-progress`
- When resolved, item is removed and written into spec

### 3. `decisions/NNNN-slug.md` — Why Non-Obvious Choices Were Made
- Capture reasoning behind complex architectural decisions
- Answers: "Why did we choose X over Y?"
- Written once, never modified (superseded decisions get new records)
- Examples: [0001 v0.1 feature set](docs/Yolang/decisions/0001-v0.1-feature-set.md), [0002 interpreter architecture](docs/Yolang/decisions/0002-interpreter-architecture.md)

## When Working on Features

### Phase 1: Design (if not in spec)
1. Check [Language Spec](docs/Yolang/spec/Language%20Spec.md) — is the feature already there?
2. If not, check [Backlog](docs/Yolang/spec/Backlog.md) — is it `open` or `deferred`?
3. If `open`, design it:
   - Clarify syntax, semantics, edge cases
   - Consider existing patterns (how do similar features work?)
   - Write into spec (or create/update backlog item)
   - If the design is non-obvious, file a decision record (`decisions/NNNN-slug.md`)

### Phase 2: Update Grammar (if syntax needs adding)
- Edit [grammar.pest](tree-walk-interpreter/src/grammar.pest)
- Rule naming mirrors feature name (e.g., `match_expr`, `trait_impl`)
- Grammar must have a single entry point: `program`
- **Grammar is mutable and iterated on.** Update it as feature implementation reveals gaps.

### Phase 3: Implement Parser
- Update [ast/mod.rs](tree-walk-interpreter/src/ast/mod.rs) with new node types
- Update [parser/mod.rs](tree-walk-interpreter/src/parser/mod.rs) to build nodes from CST
- Test: ensure `.yolo` files parse to correct AST

### Phase 4: Implement Type Checker
- Update [types/mod.rs](tree-walk-interpreter/src/types/mod.rs) if new types needed
- Update [typechecker/mod.rs](tree-walk-interpreter/src/typechecker/mod.rs) to type-check feature
- Update [typed_ast/mod.rs](tree-walk-interpreter/src/typed_ast/mod.rs) if typed nodes differ from untyped
- Test: ensure type errors are caught, type inference works correctly

### Phase 5: Implement Evaluator
- Update [evaluator/mod.rs](tree-walk-interpreter/src/evaluator/mod.rs) to interpret feature
- Runtime semantics must match spec exactly
- Test: run test files, verify output matches inline comments

### Phase 6: Update Error Handling
- Update [error/mod.rs](tree-walk-interpreter/src/error/mod.rs) if new error categories needed
- Ensure errors include `Span` for source location
- Use `miette` for pretty error reporting

### Phase 7: Add Tests
- Create `.yolo` test file that exercises the feature
- Include expected output as inline comments
- Test should be self-contained (setup → execute → output)
- Add to [tests/README.md](tests/README.md) mapping

## Code Conventions

### Naming
- **Types, structs, enums, traits**: `PascalCase` (e.g., `Literal`, `TypeEnv`, `Decl`)
- **Functions, methods, variables**: `snake_case` (e.g., `check_type`, `builtin_print`)
- **AST node names**: Mirror grammar rules (e.g., `match_expr` rule → `MatchExpr` node)

### Spans (source locations)
- Every AST node has a `span: Span` field (byte range in source)
- **Always** include span when creating error messages
- Use `Span` to implement precise error reporting

### Error Handling
- Unified error type in [error/mod.rs](tree-walk-interpreter/src/error/mod.rs)
- Errors carry `Span` and are pretty-printed via `miette`
- Errors do not panic (use Result<T, Error>)

### Generics & Monomorphisation
- Generics are **monomorphised at interpretation time** (mirroring a compiler)
- For each use of a generic function/type with concrete args, instantiate a concrete version
- See [decision 0002](docs/Yolang/decisions/0002-interpreter-architecture.md) for rationale
- Type checker performs monomorphisation; evaluator uses monomorphised code

### Environment & Scopes
- Type checker: type environment (`TypeEnv`) maps names → types
- Evaluator: runtime environment (`Env`) maps names → values
- Both use scope stacks for nesting (entering/exiting blocks)

## Common Patterns

### Running a feature through the pipeline
```rust
// main.rs pattern (simplified)
let source = fs::read_to_string(filename)?;
let cst = pest_parse(&source)?;           // grammar.pest
let ast = parser::build_ast(cst)?;        // parser/mod.rs
let typed_ast = typechecker::check(ast)?; // typechecker/mod.rs
let output = evaluator::eval(typed_ast)?; // evaluator/mod.rs
println!("{}", output);
```

### Adding a new expression type
1. Add rule to grammar.pest: `my_expr = { ... }`
2. Add variant to `ast::Expr`: `MyExpr { fields, span }`
3. Add parsing logic to `parser/mod.rs`
4. Add type checking to `typechecker/mod.rs` → returns `(typed_ast::MyExpr, ty::Type)`
5. Add evaluation to `evaluator/mod.rs` → returns `Value`
6. Update error.rs if new error cases exist

## Key Implementation Notes

### v0.1 Feature Freeze
The feature set is **complete and frozen**. See [0001 v0.1 feature decisions](docs/Yolang/decisions/0001-v0.1-feature-set.md) for what is in/out. Do not add features outside this set without updating the decision record.

### No Module System Yet
v0.1 has no `use` / `pub` / visibility. All code is in a single compilation unit. Multi-file programs come later.

### Type System Highlights
- Explicit nullability: `Perhaps<T>` (Some / nope), not null
- Explicit errors: `Result<T, E>`, not exceptions
- Generics with `where` clause bounds
- Traits for polymorphism (no vtables; monomorphisation)
- No operator overloading yet (deferred)

### Deferred Features
Check [Backlog](docs/Yolang/spec/Backlog.md) for what's intentionally excluded. Examples:
- Module system, visibility (`pub`), string interpolation
- UInt type, operator overloading, derived traits
- Panic recovery, fallible conversion traits

## Debugging Tips

### Parse errors
- Run `cargo run -- filename.yolo 2>&1` to see parser output
- Check grammar.pest to understand what was expected
- `miette` error display shows source context

### Type errors
- Type checker traces through untyped AST
- Enable debug output in typechecker/mod.rs (or add tracing)
- Monomorphisation happens here; check that generic instantiation is correct

### Runtime errors
- Add `println!` / `dbg!` to evaluator/mod.rs
- Check environment state (what values are in scope?)
- Ensure recursive evaluation matches spec (especially match, function calls)

### Test failures
- Expected output is in inline comments in `.yolo` files
- Run individual test: `cargo run -- tests/NN_*.yolo`
- Compare output to comment; check for subtle differences (whitespace, type names, nope vs null)

## Quick Reference Commands

```bash
# From tree-walk-interpreter/
cargo build              # Compile
cargo run -- FILE.yolo  # Run a script
cargo check             # Quick syntax check
cargo fmt               # Auto-format code
cargo clippy            # Lint
cargo test              # (If added; currently manual)
```

## Related Decisions

- [0001 v0.1 Feature Set](docs/Yolang/decisions/0001-v0.1-feature-set.md): What is / is not in v0.1
- [0002 Interpreter Architecture](docs/Yolang/decisions/0002-interpreter-architecture.md): Why tree-walking, why static type checker, why monomorphisation

---

**Last updated:** 2026-04-04  
**Spec version:** v0.1
