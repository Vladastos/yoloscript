# Yolang

A Rust-inspired programming language with a tree-walk interpreter written in Rust.

## What is Yolang?

Yolang is a statically typed, expression-oriented programming language designed with inspiration from Rust. It features:

- **Strong static typing** with local type inference
- **Algebraic data types** (enums with data-carrying variants)
- **Exhaustive pattern matching**
- **Explicit nullability** via `Perhaps<T>` (no null pointers)
- **Explicit error handling** via `Result<T, E>`
- **First-class functions** and closures
- **Generics** with compile-time monomorphization
- **Traits** for ad-hoc polymorphism
- **Memory managed by the runtime** (reference counting)

See [Language Specification](./docs/01-SPEC/LANGUAGE-SPEC.md) for the complete definition.

## Quick Start

### Prerequisites
- Rust 1.70+
- Cargo

### Building

```bash
cd tree-walk-interpreter
cargo build --release
```

### Running a Program

```bash
cargo run -- path/to/program.yolo
```

### Running Tests

```bash
# All tests
cargo test

# Type inference tests specifically
cargo test --test typeinference_tests
```

## Example Program

```yolang
fun factorial(n: Int) -> Int {
    if (n <= 1) {
        1
    } else {
        n * factorial(n - 1)
    }
}

let result = factorial(5);
// result: 120
```

## Project Structure

```
Yolang/
├── tree-walk-interpreter/      # Main interpreter implementation (Rust)
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # Library exports
│   │   ├── parser/             # Parsing (pest grammar)
│   │   ├── ast/                # Abstract syntax tree
│   │   ├── typeinference/      # Type inference engine
│   │   ├── typechecker/        # Type checking pass
│   │   ├── evaluator/          # Runtime evaluation
│   │   ├── error/              # Error types
│   │   └── types/              # Type system
│   ├── tests/                  # Integration & unit tests
│   └── Cargo.toml
│
├── docs/                        # All documentation
│   └── Yolang/
│       ├── 00-PROCESS/         # How to work on the project
│       ├── 01-SPEC/            # Language specification
│       ├── 02-ARCHITECTURE/    # Design & architecture decisions
│       ├── 03-COMPONENTS/      # Component implementation guides
│       ├── 04-PLANNING/        # Strategic roadmaps
│       └── 05-TASKS/           # Issue tracking & task breakdown
│
└── README.md                    # This file
```

## Documentation

Navigate to [docs/Yolang](./docs/Yolang/) for complete documentation:

| Folder | Purpose | Start Here |
|--------|---------|-----------|
| **00-PROCESS** | How to work on this project | [PROCESS.md](./docs/00-PROCESS/PROCESS.md) |
| **01-SPEC** | Language specification | [LANGUAGE-SPEC.md](./docs/01-SPEC/LANGUAGE-SPEC.md) |
| **02-ARCHITECTURE** | Design decisions & architecture | [INTERPRETER-DESIGN.md](./docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md) |
| **03-COMPONENTS** | Implementation guides | [typeinference/](./docs/03-COMPONENTS/typeinference/) |
| **04-PLANNING** | Roadmap & strategic plans | [MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md) |
| **05-TASKS** | Current work & issues | [epic-001-typechecker/](./docs/05-TASKS/epic-001-typechecker/) |

## Current Status

### v0.1

The v0.1 interpreter focuses on **language spec validation**: proving the specification is complete, consistent, and implementable.

**v0.1 includes:**
- Parser (PEG grammar via pest)
- AST representation
- Error handling framework
- Type system definition
- Type inference engine with let-polymorphism
- Type checking pass
- Expression evaluation
- Generics & monomorphization
- Trait system
- Standard library functions
- REPL (interactive shell)

See [MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md) for the detailed roadmap.


## Architecture Overview

```
source code
    ↓
[Parser] → CST (via pest)
    ↓
[AST Builder]
    ↓
untyped AST
    ↓
[Type Checker] → generics resolved, monomorphized
    ↓
typed AST
    ↓
[Tree-Walking Evaluator]
    ↓
result (value or error)
```

Key design decisions documented in [docs/02-ARCHITECTURE/decisions/](./docs/02-ARCHITECTURE/decisions/).

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suite
```bash
# Type inference tests
cargo test --test typeinference_tests

# Specific phase
cargo test --test typeinference_tests phase_2

# With output
cargo test --test typeinference_tests phase_2 -- --nocapture
```

## References

- **Language Spec**: [docs/01-SPEC/LANGUAGE-SPEC.md](./docs/01-SPEC/LANGUAGE-SPEC.md)
- **Process Guide**: [docs/00-PROCESS/PROCESS.md](./docs/00-PROCESS/PROCESS.md)
- **Task Convention**: [docs/00-PROCESS/TASK-CONVENTION.md](./docs/00-PROCESS/TASK-CONVENTION.md)
- **Interpreter Design**: [docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md](./docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md)
- **Type Inference Guide**: [docs/03-COMPONENTS/typeinference/](./docs/03-COMPONENTS/typeinference/)
- **Architecture Decisions**: [docs/02-ARCHITECTURE/decisions/](./docs/02-ARCHITECTURE/decisions/)
- **Roadmap**: [docs/04-PLANNING/MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md)
- **Current Tasks**: [docs/05-TASKS/](./docs/05-TASKS/)

## License
