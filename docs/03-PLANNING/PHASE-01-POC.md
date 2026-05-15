# Phase 01: Proof-of-Concept Interpreter

## Goal

Build a usable interpreter for a defined subset of Yoloscript — and use it to iteratively stabilise the spec for that subset.

The spec is not complete at the start of this phase. It defines the intended feature set, but real usage of the interpreter will reveal gaps, wrong assumptions, and design issues that pure design work misses. The interpreter and the spec are developed in parallel: each iteration refines both.

Phase 01 delivers two successive interpreter versions:

- **v0.1** — core language without generics or traits. Fully usable for non-generic programs.
- **v0.2** — adds user-defined generics and the trait system on top of the v0.1 interpreter.

Once v0.2 is complete, usable for real programs, and its spec is stable, Phase 01 is done. Phase 02 then builds a production-quality implementation from the validated spec.

---

## Iteration Model

Phase 01 uses an agile-like loop applied to the spec itself:

```
Define feature in spec
        ↓
Implement in interpreter
        ↓
Write real programs using it
        ↓
Observe gaps, wrong assumptions, usability issues
        ↓
Refine spec
        ↓
Implement refinement
        ↓
Next feature  (repeat)
```

The interpreter is the feedback mechanism for the spec, not just a deliverable. An issue found through usage is treated the same as an implementation finding: stop, update the spec, then continue.

The PROCESS.md spec-first rule still holds within each iteration — no code diverges from the spec — but the spec itself is expected to evolve through usage.

---

## Success Criterion

The v0.2 feature set is fully implemented, the interpreter can execute real Yoloscript programs, and the spec for that feature set is stable — no known gaps or open design questions. All implemented spec sections are tagged `> ✓ Interpreter-validated (v0.1)` or `> ✓ Interpreter-validated (v0.2)` as appropriate.

---

## Milestones

| # | Milestone | Version | Status |
|---|-----------|---------|--------|
| M1 | All v0.1 test programs parse without error | v0.1 | Done |
| M2 | All v0.1 test programs type-check without error | v0.1 | In progress |
| M3 | All v0.1 test programs execute with correct output | v0.1 | Not started |
| M4 | v0.1 interpreter usable for real programs; v0.1 spec stable | v0.1 | Not started |
| M5 | All v0.2 test programs type-check and execute correctly | v0.2 | Not started |
| M6 | v0.2 interpreter usable for real programs; v0.2 spec stable | v0.2 | Not started |

M2 is tracked in epic-005 (typechecker integration). M3 requires epics 001 (typechecker) and 002 (evaluator). M5 additionally requires epics 003 (generics) and 004 (traits). M4 and M6 are the usage and stabilisation loops — not purely technical gates.

---

## Scope

### v0.1 feature set

- PEG parser (stub quality — correct AST, no error recovery)
- Hindley-Milner type checker with let-polymorphism
- Tree-walk evaluator
- Basic error reporting with source locations
- All language features in the spec **except** generics (§3.6) and traits (§10)
- Provisional implementations of trait-dependent features:
  - `for-in` over `T[]` and `Range` only (hardcoded; no `Iterable<T>`)
  - `as` for `Int ↔ Float` only (no user-defined `From` impls)
  - `?` with exact error type match only (no `From`-based coercion)

### v0.2 feature set (adds to v0.1)

- User-defined generic functions and types (§3.6)
- Full trait system: definitions, `impl Trait for Type`, trait bounds (§10)
- `Iterable<T>` trait — user-defined iterables in `for-in`
- `From<T>` trait — user-defined casts via `as` and `From`-based `?` coercion

### Explicitly deferred to Phase 02

- Parser error recovery and diagnostic suggestions
- Performance optimisation at any stage
- LLVM or bytecode compiler backend
- Standard library (begins after Phase 01 completes)
- Production-quality error messages

---

## Test Programs

Two sets of test programs track progress against each version:

- **v0.1 programs** (`tests/test_programs/v01/`) — exercise the v0.1 feature set only. No generic functions, no user-defined traits.
- **v0.2 programs** (`tests/test_programs/v02/`) — exercise generics and traits on top of the v0.1 foundation.

The original 10 programs (milestone M1) are v0.1 programs. Additional programs are added as features are implemented.

---

## Design Philosophy

All components are built with the understanding that they will likely be rewritten in Phase 02 once the language has stabilised:

- Prioritise **speed of iteration** over architectural perfection
- Focus on **correctness**, not optimisation
- Write code that is easy to modify as the spec evolves
- Do not over-engineer for extensibility — let the design emerge from usage

The AST (`src/ast/`) is the one exception: it is the contract between parser and type checker, and is designed to remain stable even if other components are rewritten.

---

## Next Phase

Phase 02 builds a production-quality implementation — compiler or optimised interpreter — using the stable, fully interpreter-validated v0.2 spec as its ground truth. Scope and approach are defined when Phase 01 completes.
