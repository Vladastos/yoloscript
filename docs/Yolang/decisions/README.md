# Yolang Design Decisions

Decision records capture the *reasoning* behind non-obvious design choices — context, alternatives considered, and why we chose what we chose. They are written once and not modified. Superseded decisions get a new record that references the old one.

See `PROCESS.md` for when to write a decision record and when not to.

**What belongs here:** why a choice was made, what alternatives were rejected and why.
**What does not belong here:** what a feature does (that is in `Language Spec.md`), what is still open (that is in `Backlog.md`).

## Statuses

- **Accepted** — in effect
- **Superseded** — replaced by a later decision (link provided)
- **Rejected** — considered but not adopted; kept for historical context

## Index

| # | Title | Status |
|---|-------|--------|
| [0001](./0001-v0.1-feature-set.md) | v0.1 Feature Set Scope | Accepted |
| [0002](./0002-interpreter-architecture.md) | Interpreter Architecture | Accepted |

## File naming

`NNNN-short-slug.md` — zero-padded number, then a kebab-case slug. Numbers are assigned sequentially and never reused.

## Template

See [`0000-template.md`](./0000-template.md).
