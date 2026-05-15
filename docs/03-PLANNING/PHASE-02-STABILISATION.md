# Phase 02: Language Stabilisation

**Status:** Not started. Begins when Phase 01 (v0.1 + v0.2) completes.

---

## Goal

Move from a working proof-of-concept interpreter to a **stable, well-defined language specification** — one that is ready to serve as the ground truth for a production implementation.

Phase 01 produces a spec that is correct and usable, but shaped primarily by one developer and one implementation. Phase 02 stress-tests that spec against real usage, external feedback, and edge cases that only emerge when the language is used seriously. The spec exits this phase with no known gaps, no open design questions, and a defined process for how it evolves going forward.

---

## Why a Separate Phase?

A PoC interpreter built by one person against one implementation naturally has blind spots:

- **Design decisions made in isolation** — choices that seemed obvious may not hold up when others use the language or when programs grow beyond toy examples.
- **Missing edge cases** — spec sections often look complete until someone tries to use a feature in an unexpected combination.
- **Implicit assumptions** — things the author "just knows" that are not written down.
- **Missing features** — real programs reveal what the language still needs.

Phase 02 exists to surface and resolve all of the above before committing to the cost of a production implementation.

---

## Activities

### 1. Structured usage and feedback collection

Run the interpreter against a wider range of programs — not just the test suite. This includes programs written by other developers if possible. Track friction points, surprising behaviours, and missing features.

A lightweight feedback process should be defined at the start of this phase (see §Language Definition Process below).

### 2. Spec gap identification

Audit every spec section against real interpreter usage. For each section:

- Does the description match what the interpreter actually does?
- Are all edge cases covered?
- Are there combinations of features not addressed?
- Is the wording precise enough to implement without asking the author?

Tag completed sections `> ✓ Interpreter-validated (v0.1)` or `> ✓ Interpreter-validated (v0.2)`.

### 3. Feature additions and design changes

Real usage will likely reveal missing features or design decisions that need revisiting. These flow through the same spec-first process as Phase 01: define in spec, implement, validate, refine.

Candidates expected going into this phase:
- `math` module (`floor`, `ceil`, `abs`, `sqrt`, `pow`, `min`, `max`)
- `string` module (`split`, `trim`, `contains`, `to_upper`, `to_lower`)
- `io` module (`read_line`, `read_file`, `write_file`)
- Integer overflow behaviour (panic vs. wrapping — currently open in backlog)
- String interpolation (syntax not yet decided — in backlog)
- Operator overloading traits

### 4. Language definition process

Phase 02 should define — and begin operating under — a formal process for how the Yoloscript language evolves. At minimum this covers:

- **How changes are proposed** — a lightweight RFC or proposal format.
- **How proposals are evaluated** — criteria for accepting, deferring, or rejecting changes.
- **Breaking vs. non-breaking changes** — what constitutes a breaking change and how it is handled.
- **Stability guarantees** — what users can rely on not changing between versions.
- **Who decides** — decision-making authority, especially if external contributors are involved.

This process does not need to be heavyweight for a language at this stage, but it should be explicit enough that contributors understand the rules and the spec remains coherent.

---

## Success Criterion

The language spec is stable:

- No known gaps or open design questions.
- All spec sections tagged as interpreter-validated.
- The language definition process is documented and operational.
- At least one real program of non-trivial complexity runs correctly and the experience of writing it has been used to refine the spec.

---

## Milestones

| # | Milestone | Status |
|---|-----------|--------|
| M1 | Feedback process defined; external usage begun | Not started |
| M2 | Spec audit complete; all gaps identified and tracked | Not started |
| M3 | All identified gaps resolved; spec sections validated | Not started |
| M4 | Language definition process documented and operational | Not started |
| M5 | Spec declared stable; Phase 03 prerequisites met | Not started |

---

## Prerequisites

- All Phase 01 milestones complete (v0.1 and v0.2 interpreter working)
- No open high-priority spec issues carried over from Phase 01
