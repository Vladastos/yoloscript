# Task 0003: Stage 3 — Composite Expressions

**Status:** done  
**Epic:** epic-005-typechecker-integration  
**Component:** typechecker  
**Spec Link:** docs/01-SPEC/LANGUAGE-SPEC.md (§5 Expressions)  
**Blocked By:** 0002  
**Decisions:** ADR-0002

## What

Extend inference and construction to composite expression forms: `Expr::Tuple`,
`Expr::Array`, `Expr::Call`, and `Expr::Index`. After this stage, function calls and
collection construction are fully typed.

## Decisions

- **Let-polymorphism in Pass 1**: After `infer_fun_decl` completes, immediately
  call `ctx.solve()` on all constraints collected so far, apply the result to
  `fun_ty`, generalize, and `ctx.bind_poly(name, scheme)`. Future call-site
  `lookup`s then return a fresh instantiation (via `InferContext`'s existing
  poly-env path) rather than the shared hoisted variable. The full constraint set
  is unchanged — the mid-pass solve is read-only. This is Algorithm W's inline
  generalization applied within the two-pass design.
- **Non-function callee**: E0001 (type mismatch). No new error code needed.
- **Empty array literal**: annotation required. Without expected type context, E0002
  in Pass 2 (same path as `nope`). Pass 2 uses `expected_ty` to recover the element
  type when an annotation is present.

## Acceptance Criteria

- [x] `Expr::Tuple`: each element inferred; result type is `Tuple(t1, t2, ...)`;
  Pass 2 constructs `TypedExpr::Tuple`
- [x] `Expr::Array`: all elements unified to a common element type; result is
  `Array(T)`; empty array literal requires annotation (E0002 otherwise); Pass 2
  constructs `TypedExpr::Array`
- [x] `Expr::Call`: callee must be `Fun(params, ret)`; each argument unified with the
  corresponding parameter type; arity mismatch produces E0004; non-function callee
  produces E0001; result type is `ret`; Pass 2 constructs `TypedExpr::Call`
- [x] `Expr::Call` on a polymorphic function: inline solve-and-generalize in
  `infer_fun_decl` ensures each call site instantiates a fresh copy; Pass 2
  re-instantiates the scheme against concrete argument types
- [x] `Expr::Index`: operand must be `Array(T)`, index must be `Int`; result is `T`;
  Pass 2 constructs `TypedExpr::Index`
- [x] Stage 3 `.yolo` test programs pass through `check()` without error
- [x] All Stage 1–2 tests still pass

## Notes

- `InferContext::solve` changed from `self` to `&self` (clones the constraint set) so
  it can be called mid-pass in `infer_fun_decl` without consuming the context.
- `construct_fun_decl` now uses `erase_free_vars` (replaces unresolved `InferType::Var`
  with `Type::Named("?tN", [])` placeholders) rather than failing with E0002 when a
  polymorphic function's scheme has free variables. The evaluator uses runtime values,
  not type annotations, so placeholders are safe for the body.
- Test path migrated to `tests/typechecking/sources/` to match the new directory layout.
