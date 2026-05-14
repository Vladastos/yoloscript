# Task 0007: Unify `if` as a Single Expression Form

**Status:** done
**Epic:** epic-001-typechecker
**Component:** parser, typechecker
**Spec Link:** `../01-SPEC/LANGUAGE-SPEC.md` §4 Statements, §5 Expressions
**Blocked By:** none
**Decisions:** [ADR-0005](../../05-DECISIONS/closed/ADR-0005-if-expression-vs-statement.md)

## What

Remove `if_stmt` from the grammar and AST entirely. All `if` constructs become `Expr::If`
with an optional else branch. An `if` in statement position is wrapped in `Stmt::Expr` and
its value discarded — the same as any expression-statement. An `if` without an else branch
has type `Unit`; its then-branch must also produce `Unit`.

This eliminates the PEG greedy-parsing bug that prevents `if` from appearing as a block
tail (see ADR-0005), and removes the `Stmt::If` / `Expr::If` split from every layer of
the pipeline.

## Acceptance Criteria

- [x] `grammar.pest`: `if_stmt` rule removed; `if_expr` else branch made optional:
  `("else" ~ (if_expr | block))?`
- [x] `ast/mod.rs`: `Stmt::If`, `IfStmt`, and `ElseBranch` removed; `Expr::If.else_branch`
  changed to `Option<Block>`
- [x] `typed_ast/mod.rs`: `TypedStmt::If` and `TypedIfStmt` removed;
  `TypedExpr::If.else_branch` changed to `Option<TypedBlock>`
- [x] `parser/mod.rs`: all `if` constructs produce `Expr::If`; `if` in statement position
  becomes `Stmt::Expr(Expr::If { ... })`
- [x] `typechecker/mod.rs`: `infer_if_stmt` and `construct_if_stmt` removed; `infer_expr`
  and `construct_expr` for `Expr::If` handle the optional else — no-else produces `Unit`
  and requires the then-branch to also produce `Unit`
- [x] All existing Stage 2 tests still pass (if-as-statement remains valid)
- [x] New positive test: `if` used directly as a block tail type-checks correctly
  (`stage3_03_if_as_block_tail.yolo`: `fun max(a: Int, b: Int) -> Int { if (a > b) { a } else { b } }`)
- [x] New negative test: `if` without else where then-branch is non-`Unit` produces E0001
  (`stage3_neg_07_if_no_else_non_unit.yolo`)

## Notes

### Implementation

Statement-position `if` now requires a trailing `;` (parsed as `expr_stmt`). All existing
test sources in `tests/parsing/sources/` and `tests/typechecking/sources/` were updated
accordingly. An `else if` chain is represented by wrapping the nested `if_expr` in a
synthetic `Block` with no stmts and the `if_expr` as the tail, so `Expr::If.else_branch`
is always `Option<Block>` with no separate chain variant.

All 125 tests pass.
