# ADR-0005: if-Statement vs if-Expression — Grammar and AST Unification

**Status:** accepted  
**Date:** 2026-05-10  

## Context

The current grammar has two separate rules for `if`:

```pest
if_stmt = { "if" ~ "(" ~ expr ~ ")" ~ block ~ ("else" ~ (if_stmt | block))? }
if_expr = { "if" ~ "(" ~ expr ~ ")" ~ block ~ "else" ~ block }
```

`if_stmt` is a `stmt`, which is a `decl`. `if_expr` is a `primary_expr`.

The block rule is:

```pest
block = { "{" ~ decl* ~ expr? ~ "}" }
```

Because PEG parsers are greedy and `decl*` is tried before the optional trailing
`expr`, any `if` that appears inside a block is always consumed as `Decl::Stmt(Stmt::If(…))`
— the parser never reaches the `expr?` slot to try it as `if_expr`.

**Consequence:** `Expr::If` can never be the tail of a block. It only appears in
genuine expression positions — let binding values, function arguments, binary operands,
etc. The following idiomatic pattern fails to type-check:

```yoloscript
fun max(a: Int, b: Int) -> Int {
    if (a > b) { a } else { b }  // parsed as Stmt::If; block tail is None → Unit
}                                 // E0001: cannot unify Unit with Int
```

The workaround is to assign the if-expression to a let binding:

```yoloscript
fun max(a: Int, b: Int) -> Int {
    let result: Int = if (a > b) { a } else { b };
    result
}
```

This is surprising for users and inconsistent with Rust, whose design Yoloscript borrows
from heavily. In Rust, `if` is always an expression and can appear directly as a
function body's tail value.

Discovered during Stage 2 typechecker implementation (epic-005, task 0002).

## Options Considered

### Option A: Unify if-statement and if-expression into a single AST node

Remove `if_stmt` from the grammar and AST entirely. All `if` constructs are parsed
as `if_expr` (an expression). An `if` in statement position becomes
`Decl::Stmt(Stmt::Expr(Expr::If { … }))` — the value is discarded, as with any
expression-statement.

The `else_branch` in `Expr::If` becomes `Option<Block>` (currently it is a required
`Block`). An `if` without `else` has type `Unit` when used as an expression (since
control may or may not enter the branch). An `if` with `else` has the unified type of
both branches.

Grammar change:
```pest
if_expr = { "if" ~ "(" ~ expr ~ ")" ~ block ~ ("else" ~ (if_expr | block))? }
// if_stmt removed entirely; Stmt::If removed from AST
```

AST change: `Stmt::If` and `TypedStmt::If` are removed. The grammar's `else_branch`
moves entirely into `Expr::If`.

**Pros:**
- Removes the limitation permanently — `if` works in all positions including block tails
- Consistent with Rust semantics; matches user expectations
- Simplifies the grammar (one rule instead of two)
- Typechecker simplifies: one code path handles all `if` forms

**Cons:**
- Significant refactor: grammar, parser, AST, typed AST, typechecker (both passes),
  and eventually the evaluator all need changes
- `Stmt::If` removal affects any evaluator code already written for it
- `Expr::If` with optional else changes the typechecker: must produce `Unit` when
  no else branch, rather than erroring on missing else

### Option B: Reorder the block grammar to try expr? before decl* for trailing if

Change `block` to use negative look-ahead or parser ordering so that a trailing
`if...else...` is tried as an `expr` before being consumed as a `decl`:

```pest
block = { "{" ~ non_if_decl* ~ expr? ~ "}" | "{" ~ decl* ~ "}" }
```

Or use a cut / ordered choice to prevent `if_stmt` from consuming a trailing
`if` that would otherwise be a usable expression.

**Pros:**
- Preserves the `Stmt::If` / `Expr::If` split; smaller change
- No AST changes required

**Cons:**
- PEG grammars have no true look-ahead for this pattern without significant restructuring
- Fragile: the same problem would recur for any new construct added as both a statement
  and an expression (e.g. `loop`, `match`)
- Does not fix the fundamental design issue — just patches one symptom

### Option C: Accept the limitation and document it

Keep the grammar as-is. Specify in the language spec that `if` at the tail of a block
is always a statement; the workaround is `let result = if (…) { … } else { … }`.

**Pros:**
- No implementation work
- Grammar remains simple and unambiguous

**Cons:**
- Surprising to users; diverges from the Rust design Yoloscript follows
- The workaround adds noise (extra `let` binding) to every function that wants to
  return an if-expression
- The doc for the grammar quirk is in the notes of an epic-005 task — easy to lose

## Decision

**Option A** — unify `if` into a single expression form.

`if_stmt` is removed from the grammar and AST. All `if` constructs are `Expr::If`. An `if` in statement position is wrapped in `Stmt::Expr`; its value is discarded. An `if` without an `else` branch has type `Unit`; the then-branch must also produce `Unit`.

## Implementation Notes

**Date implemented:** 2026-05-14

### The statement-position semicolon problem

A naïve first attempt removed `if_stmt` from `stmt` and moved `if_expr` into `stmt` directly. This required a trailing `;` when `if` appeared in statement position (to close the `Stmt::Expr` like any other expression-statement). This was rejected — the original grammar never required a semicolon after `if`, and requiring one would be a visible regression.

### Solution: negative lookahead in the block grammar

The block rule was restructured to distinguish statement-position `if` from tail-position `if` without a semicolon:

```pest
block        = { "{" ~ block_item* ~ expr? ~ "}" }
// block_item wraps each statement-level item. if_stmt_item handles `if` in
// statement position without a trailing `;`: it matches only when the `if`
// expression is NOT immediately followed by the closing `}` of the block,
// leaving that case for the tail `expr?`.
block_item   = { if_stmt_item | decl }
if_stmt_item = { if_expr ~ !"}" }
```

How it works:
- `block_item*` is tried repeatedly. Each iteration tries `if_stmt_item` first.
- `if_stmt_item` matches an `if_expr` only when the token immediately after (skipping whitespace) is **not** `}`.
- When `if` is the last thing before `}`, `if_stmt_item` fails, `decl` also fails, `block_item*` stops, and `expr?` picks up the `if_expr` as the block tail — no semicolon needed at any point.
- When `if` is followed by more statements, `if_stmt_item` succeeds and wraps it as `Decl::Stmt(Stmt::Expr(Expr::If { … }))`.

### else if chaining

`else if` is represented by wrapping the nested `if_expr` in a synthetic `Block`:

```rust
Rule::if_expr => {
    let nested = parse_if_expr(p, filename)?;
    let else_span = nested.span().clone();
    Block { stmts: vec![], tail: Some(Box::new(nested)), span: else_span }
}
```

This keeps `Expr::If.else_branch: Option<Block>` with no special chain variant in the AST or typed AST.

### Files changed

- `src/grammar.pest` — removed `if_stmt`; updated `if_expr` to optional else; added `block_item` and `if_stmt_item`
- `src/ast/mod.rs` — removed `Stmt::If`, `IfStmt`, `ElseBranch`; changed `Expr::If.else_branch` to `Option<Block>`
- `src/typed_ast/mod.rs` — removed `TypedStmt::If`, `TypedIfStmt`, `TypedElseBranch`; changed `TypedExpr::If.else_branch` to `Option<TypedBlock>`
- `src/parser/mod.rs` — removed `parse_if_stmt`; updated `parse_if_expr` for optional else and `else if` desugaring; updated `parse_block` for `block_item`/`if_stmt_item`
- `src/typechecker/mod.rs` — removed `infer_if_stmt`, `construct_if_stmt`; updated `infer_expr` and `construct_expr` for `Expr::If` with `Option<Block>`

## Consequences

- `Stmt::If`, `IfStmt`, `ElseBranch`, `TypedStmt::If`, and `TypedIfStmt` are removed
- Grammar: `if_stmt` rule deleted; `if_expr` gains optional else (`("else" ~ (if_expr | block))?`); `block` uses `block_item*` with a negative-lookahead `if_stmt_item` to avoid requiring semicolons
- Parser: all `if` constructs produce `Expr::If`; statement position wraps it in `Stmt::Expr` (no semicolon required)
- Typechecker: `infer_if_stmt` and `construct_if_stmt` removed; `infer_expr`/`construct_expr` for `Expr::If` handle the optional `else_branch`; no-else `if` produces `Unit` and requires the then-branch to also produce `Unit`
- Evaluator: `Expr::If` with absent else branch evaluates to unit when the condition is false
- `if` used directly as a block tail now type-checks correctly — the `fun max` pattern works without a `let` workaround

## References

- Task: [0002 — Stage 2: Control Flow](../../04-TASKS/epic-005-typechecker-integration/done/0002-stage2-control-flow.md) (where the limitation was discovered)
- Spec: [§4 — Statements](../../01-SPEC/LANGUAGE-SPEC.md#4-statements), [§5 — Expressions](../../01-SPEC/LANGUAGE-SPEC.md#5-expressions)
