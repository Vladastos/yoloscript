# Yolang Medium-Term Development Plan

## Overview
This document outlines the development strategy for the core components of the Yolang language during the proof-of-concept phase. The goal is to build a working interpreter that validates the language specification, not to construct production-quality components. Once the language spec is stable and proven through the PoC interpreter, a production implementation can be built from lessons learned.

---

## Component Development Strategy

### 1. Parser — **STUB** ⚠️

**Status:** Stub implementation only

**Scope (Current Iteration):**
- Basic parsing of language syntax using `pest`
- Generate correct AST structure
- Support core language features

**Out of Scope (Deferred to Future Iterations):**
- Error reporting and diagnostics
- Error recovery mechanisms
- Quality-of-life features (suggestions, contextual hints, etc.)
- Performance optimization

**Rationale:**
The parser is a stub because error handling, recovery, and UX features can be added later without requiring major rewrites. The critical requirement is that the parser produces a stable AST that aligns with the Evaluator's expectations.

---

### 2. AST (Abstract Syntax Tree) — **ALIGNED & STABLE** ✅

**Status:** Designed to last; must not require rewriting during interpreter implementation

**Design Principles:**
- AST structure must directly support the Evaluator's needs
- Should represent the language semantically, not syntactically
- Must be flexible enough to accommodate future language spec additions without structural rewrites
- All parsing logic produces this AST; any parser rewrites should not affect AST structure

**Key Consideration:**
The AST is the contract between the Parser and the Evaluator. Once established, the AST should remain stable even if the parser is rewritten or enhanced with better error recovery.

---

### 3. Type System & Type Checker — **PROOF-OF-CONCEPT** 🧪

**Status:** Functional but not production-quality; will likely be rewritten

**Requirements:**
- Implement a working type system that validates the type checking rules in the language specification
- Handle the type features defined in the current spec
- Be simple enough to iterate on as the spec evolves

**Design Considerations:**
- Focus on correctness and clarity, not optimization or architectural perfection
- Build it in a way that makes it easy to rewrite once the spec stabilizes
- Don't over-engineer for extensibility; discover what works through experimentation
- Rewrite when the spec has solidified and lessons from the PoC are clear

**Expected Outcome:**
This type checker proves the type system design is sound. Once the language spec is stable, a production version will be built with those lessons in mind.

---

### 4. Interpreter/Evaluator — **PROOF-OF-CONCEPT** 🧪

**Status:** Functional reference implementation; will be refined as spec stabilizes

**Requirements:**
- Implement a working evaluator that correctly executes the AST
- Handle all language semantics defined in the current specification
- Be simple and straightforward; prioritize clarity over optimization

**Design Considerations:**
- Write it to be easy to understand and modify as the language spec evolves
- Don't optimize prematurely; focus on correctness
- Expect to refactor or rewrite portions as new language features are added
- Use the PoC to discover what actually works before committing to a final design

**Expected Outcome:**
This interpreter proves the language semantics work as intended. Once the spec stabilizes, a production interpreter (potentially with optimization or compilation targets) will be built.

---

### 5. Standard Library & Built-in Functions — **DEFERRED** ⏸️

**Status:** Not started; planned for post-1.0 stable release

**Rationale:**
The standard library will be developed only after a first stable version of the language and interpreter is available. This allows:
- The language spec and semantics to stabilize first
- Clear understanding of the runtime model and capabilities
- Opportunity to design a clean, cohesive standard library API

**Future Timing:**
Standard library development begins once the interpreter reaches feature parity with the language specification and can reliably execute non-trivial programs.

---

### 6. LLVM Compiler Backend — **FUTURE** 🚀

**Status:** Not in current scope; planned for later phase

**Note:**
The Evaluator and AST design should not preclude future compilation. However, no work on LLVM integration is planned for this iteration.

---

## Implementation Philosophy

### Proof-of-Concept Mindset
All components should be built with the understanding that they will likely be rewritten once the language stabilizes:
- Prioritize **speed of iteration** over architectural perfection
- Focus on **correctness**, not optimization
- Write code that's easy to modify and refactor
- Don't over-design for extensibility; let the design emerge from the spec

### AST as a Working Contract
The AST is the interface between Parser and Evaluator:
- Should change as needed to support new language features
- Parser may be rewritten entirely; AST evolves with the spec
- Both parser and evaluator should be easy to update when AST changes

---

## Next Steps

1. **Continue language spec development** — this is the primary driver
2. **Implement parser stubs** that produce correct AST for current spec
3. **Build PoC type checker** that validates the type system as designed
4. **Build PoC interpreter** that executes the language semantics
5. **Iterate**: As spec changes, update parser, type checker, and interpreter
6. **Defer refinement** of all components until spec stabilizes
7. **Plan production implementation** once language is proven viable

