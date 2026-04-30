# Task 0010: Let-Polymorphism Support

**Status:**      open  
**Epic:**        epic-003-generics  
**Component:**   typechecker  
**Spec Link:**   spec/Backlog.md (language features — open design)  
**Blocked By:**  0005 (Type variables and constraints)

## What

Enable generic closures bound to `let` to be used polymorphically with different types, without explicit type parameter annotations.

Currently, Yolang supports explicit generics on named functions:

```yolang
fun id<T>(x: T) -> T { x }

let my_id = id;
my_id(42);         // Works
my_id("hello");    // Works (different instantiation)
```

But it's unclear whether implicit polymorphism works for let-bound closures without explicit `<T>` parameters:

```yolang
let id = fun(x) { x };  // No explicit <T>
id(42);                 // Should infer T = Int
id("hello");           // Should infer T = String (polymorphic reuse?)
```

This task clarifies the design and implements support for let-bound polymorphic closures via:

1. **Type scheme generation** — Convert inferred function types to polymorphic schemes
2. **Generalization** — When binding a generic function, identify quantified type variables
3. **Instantiation** — When using a polymorphic binding, create fresh type variables for each use
4. **Constraint-based inference** — Unification determines the concrete type at each call site

## Design Decision (DECIDED)

✅ **Yes, Yolang supports let-polymorphism.** Implicit generalization is enabled: let-bound closures can be used polymorphically without explicit `<T>` parameters, using Hindley-Milner style generalization + instantiation.

## Acceptance Criteria

- [ ] Type schemes (`∀α. α → α`) represented in typechecker
- [ ] Generalization algorithm converts inferred types to schemes on binding
- [ ] Instantiation creates fresh type variables on each use
- [ ] Let-bound closures can be called with multiple concrete types
- [ ] Example works: `let id = fun(x) { x }; id(42); id("hello");`
- [ ] Type variable unification resolves concrete types at call sites
- [ ] Error messages distinguish between monomorphic and polymorphic constraint violations
- [ ] Test suite covers let-polymorphic functions, nested generics, recursive polymorphic functions, and error cases

## Notes

**Depends on:** Task 0005 (type variables and constraint system) — must have unification working first.

**Implementation considerations:**
- All let-bound closures are polymorphic (generalization happens on binding)
- Type annotations constrain but don't prevent polymorphism: `let id: fun(Int) -> Int = fun(x) { x };` binds `id` to monomorphic version
- Mutually recursive functions: handle via simultaneous generalization in the constraint solver
- Interaction with explicit generics: `fun id<T>(x: T)` and let-bound `id` are treated similarly (both polymorphic)

**Related to:** Epic 001 (type inference), Epic 003 (generics and monomorphization). This task bridges implicit inference (Hindley-Milner style) with explicit generics (Rust style).

## References

- Hindley-Milner algorithm and let-polymorphism: polymorphic generalization
- Constraint-based type inference: unification-driven instantiation
- Modern implementations: OCaml, Haskell (both support let-polymorphism)
