# Tasks

Tasks are concrete units of work. Each task file tracks one piece of work from start to finish.

## Quick Start

1. Copy `0000-template.md` → rename to `NNNN-slug.md` (next ID)
2. Fill in: **Status**, **What**, **Acceptance Criteria**
3. Link to related **Spec Section** or **Backlog Item**
4. Done

## Task Format

```markdown
# Task NNNN: Brief Title

**Status:** open | in-progress | done | blocked

**Component:** interpreter | repl | parser | typechecker | evaluator | error-handling | spec

**Spec Link:** `../01-SPEC/LANGUAGE-SPEC.md#Section-Name` (or Backlog item if not yet speced)

**Blocked By:** (if applicable)

## What

One or two sentences. What needs doing and why.

## Acceptance Criteria

- [ ] Clear, testable outcome 1
- [ ] Clear, testable outcome 2
- [ ] No test regressions

## Notes

(Optional) Progress, discoveries, decisions made during work.
```

## Example

```markdown
# Task 0001: Implement error recovery in parser

**Status:** in-progress

**Component:** parser

**Spec Link:** `../01-SPEC/LANGUAGE-SPEC.md#Error-Recovery`

**Blocked By:** none

## What

The parser currently panics on unexpected tokens. Implement error recovery so it can report multiple errors and continue parsing. This unblocks better error messages and REPL debugging.

## Acceptance Criteria

- [ ] Parser continues after syntax errors (doesn't panic)
- [ ] Reports at least 3 syntax errors per file
- [ ] Error position (line:col) is accurate
- [ ] All existing tests still pass

## Notes

- Started 2026-04-07
- Using the panic recovery pattern from the Crafting Interpreters book
- Found that we need to track token boundaries better for recovery
```

## Status

- **open** — ready to start
- **in-progress** — actively being worked
- **done** — finished, verified, spec updated if needed
- **blocked** — waiting for something else

## Components

Link each task to the main component it affects:

- **interpreter** — overall interpreter, tree-walk evaluator
- **repl** — interactive shell, debugging features
- **parser** — parser, grammar, lexer
- **typechecker** — type inference, type checking
- **evaluator** — runtime, environment, values
- **error-handling** — error messages, recovery, diagnostics
- **spec** — language specification work

## Linking

Every task should link to the spec section it implements/validates:

```markdown
**Spec Link:** `../01-SPEC/LANGUAGE-SPEC.md#Section-Name`
```

If the feature isn't speced yet, link to the backlog:

```markdown
**Spec Link:** Backlog item "Feature Name"
```

## Workflow

1. **Create task** → Set status to `open`
2. **Start work** → Change status to `in-progress`, add notes as you go
3. **Finish** → Check all criteria, update spec if needed, set status to `done`
4. **If blocked** → Set status to `blocked` with reason, update when unblocked

## Keep It Honest

- If status is `in-progress` and you haven't touched it in days → mark as `blocked` with reason
- If scope changed → update the criteria
- If you found something in the spec that needs clarifying → note it
- Don't let stale tasks pile up

## That's It

No complex metadata, no excessive linking, no separate process documents. Just:
- Clear title and description
- Link to what you're implementing
- Clear "done" criteria
- Status that reflects reality
- Notes if you discover anything

See `0000-template.md` to create a new task.
