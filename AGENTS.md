# AGENTS.md

Quick entry point for AI agents (Claude, etc.) contributing to Yolang.

## Essential Rules

1. **The spec is law** - `docs/01-SPEC/LANGUAGE-SPEC.md` is the source of truth
2. **Always check existing tasks** - `docs/05-TASKS/` before starting
3. **Write tests before code** - All implementation must have tests
4. **Update references** - When moving files, update all cross-references
5. **Ask before creating documentation** - All .md files must follow the established structure

---

## How to Contribute

Follow the same process as human contributors:

1. **Read** `docs/00-PROCESS/PROCESS.md` - How the project works
2. **Pick a task** - From `docs/05-TASKS/`
3. **Follow** `docs/00-PROCESS/TASK-CONVENTION.md` - Task conventions
4. **Reference** `docs/01-SPEC/LANGUAGE-SPEC.md` - The spec is law
5. **Test thoroughly** - Write tests before implementing
6. **Update docs** - Keep documentation current

---

## Key Locations

- **Spec** - `docs/01-SPEC/LANGUAGE-SPEC.md` (source of truth)
- **Process** - `docs/00-PROCESS/`
- **Tasks** - `docs/05-TASKS/`
- **Code** - `tree-walk-interpreter/src/`
- **Tests** - `tree-walk-interpreter/tests/`

---

## Quick Start

```bash
cd tree-walk-interpreter
cargo test          # Run all tests
cargo test --test typeinference_tests phase_2  # Run specific tests
```

---

**This is an entry point.** Full instructions for all contributors are in `docs/00-PROCESS/`.
