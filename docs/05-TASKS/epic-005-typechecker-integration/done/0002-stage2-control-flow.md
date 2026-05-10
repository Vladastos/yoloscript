# Task 0002: Stage 2 — Control Flow Statements

**Status:** done  
**Epic:** epic-005-typechecker-integration  
**Component:** typechecker  
**Spec Link:** docs/01-SPEC/LANGUAGE-SPEC.md (§4 Statements, §5 Expressions)  
**Blocked By:** 0001  
**Decisions:** ADR-0002

## What

Extend the two-pass typechecker to handle control flow: `Stmt::If`, `Stmt::While`,
`Stmt::Return`, and `Expr::If` (if-as-expression). After this stage the typechecker
handles all branching and looping constructs that don't require pattern matching or
closures.

## Acceptance Criteria

- [x] `Stmt::If`: condition constrained to `Bool`; then/else blocks inferred; Pass 2
  constructs `TypedStmt::If`
- [x] `Stmt::While`: condition constrained to `Bool`; body inferred; Pass 2 constructs
  `TypedStmt::While`
- [x] `Stmt::Return`: return expression unified with `current_return_type`; bare
  `return` treated as `Unit`
- [x] `Expr::If`: both branches must unify to a common type; that type is the result;
  condition constrained to `Bool`; Pass 2 constructs `TypedExpr::If`
- [x] Stage 2 `.yolo` test programs pass through `check()` without error
- [x] Type errors in conditions produce `TypeError` with correct error code and span
- [x] All Stage 1 tests still pass

## Notes

- `infer_expr` gained a `pending: &mut Vec<PendingFun>` parameter (needed because
  `Expr::If` contains blocks that can hold `FunDecl`s). All callers updated.
- `infer_block` and `construct_block` now push/pop scope so block-local bindings
  don't leak into the enclosing scope.
- Grammar limitation: `if...else...` inside a block is always parsed as `Stmt::If`
  (via `decl*`) rather than `Expr::If`, because the PEG parser greedily tries `decl`
  first. `Expr::If` only appears in genuine expression positions (e.g. `let x = if
  (cond) { a } else { b };`). Test programs use `let` bindings to exercise `Expr::If`.
- `else if` chains are handled recursively via `ElseBranch::If`; `construct_if_stmt`
  mirrors this recursion in Pass 2.
