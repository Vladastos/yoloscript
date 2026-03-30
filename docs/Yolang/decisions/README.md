# Yolang Design Decisions

This directory tracks the design decisions made during the development of the Yolang language specification and implementation.

## Format

Each decision is a separate Markdown file named with a zero-padded number and a short slug:

```
0001-type-system.md
0002-memory-model.md
0003-error-handling.md
```

Numbers are assigned sequentially and never reused. A decision, once recorded, is not deleted — if a decision is reversed or superseded, a new entry is added that references the old one.

## Decision statuses

- **Proposed** — Under consideration, not yet settled
- **Accepted** — Agreed upon and reflected in the spec
- **Rejected** — Considered but decided against; kept for historical context
- **Superseded** — Replaced by a later decision (link to the new one)

## Template

See [`0000-template.md`](./0000-template.md) for the file template to copy when recording a new decision.
