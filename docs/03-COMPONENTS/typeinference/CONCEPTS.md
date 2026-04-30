# Type Schemes and Let-Polymorphism Design

## Overview

The typeinference module now includes **type scheme support** to enable let-polymorphism in Yolang. This allows let-bound closures to be used polymorphically with different concrete types without explicit type parameters.

## Concepts

### Type Schemes

A **type scheme** is a type with universally quantified variables:

```
∀α. α → α    (identity function — works with any type)
∀α β. α → β → α  (first function — returns first of two arguments)
Int → Int    (monomorphic type — no quantified variables)
```

In code, represented as:
```rust
pub struct TypeScheme {
    pub quantified_vars: Vec<TypeVar>,  // [?t0] for ∀α
    pub ty: InferType,                  // The body: ?t0 → ?t0
}
```

### Generalization

When you bind a polymorphic closure to `let`, its inferred type is **generalized** into a type scheme:

```yolang
let id = fun(x) { x };
```

**Inference steps:**
1. Infer the body `{ x }` has type `?t0` (parameter type)
2. Infer the function has type `fun(?t0) -> ?t0`
3. **Generalize:** Identify free type variables (variables not constrained by context)
   - `?t0` is free (not used elsewhere)
   - Becomes quantified: `∀?t0. fun(?t0) -> ?t0`
4. Bind `id` to the scheme in the polymorphic environment

**Code:**
```rust
let free_in_env = ctx.free_vars_in_environment();  // Already-constrained vars
let scheme = TypeScheme::generalize(inferred_type, &free_in_env);
ctx.bind_generalized(name, inferred_type);
```

### Instantiation

When you **use** a polymorphic binding, the scheme is instantiated with fresh type variables:

```yolang
let id = fun(x) { x };  // Scheme: ∀α. α → α
id(42);                 // First use: instantiate to ?t_fresh1 → ?t_fresh1
                        //   Unify: ?t_fresh1 = Int
id("hello");            // Second use: instantiate to ?t_fresh2 → ?t_fresh2
                        //   Unify: ?t_fresh2 = String
```

**Code:**
```rust
pub fn lookup_var(&mut self, name: &str) -> Option<InferType> {
    if let Some(scheme) = self.poly_env.get(name).cloned() {
        return Some(scheme.instantiate(self));  // Create fresh vars each time
    }
    self.env.get(name).cloned()
}

// In instantiate():
for &quantified_var in &self.quantified_vars {
    let fresh = ctx.fresh_var();  // Fresh variable for this use
    subst.bind(quantified_var, InferType::Var(fresh));
}
```

## Example: Identity Function

```yolang
let id = fun(x) { x };
let y = id(42);
let z = id("hello");
```

### Inference trace:

**1. Infer `fun(x) { x }`:**
```
Parameter x: type variable ?t0
Body { x }: type is ?t0 (the parameter)
Function type: fun(?t0) -> ?t0
```

**2. Generalize for binding:**
```
Free vars in environment: {} (empty — first binding)
All vars in fun(?t0) -> ?t0 are free
Generalized scheme: ∀?t0. fun(?t0) -> ?t0
Bind: id ↦ TypeScheme { quantified_vars: [?t0], ty: fun(?t0) -> ?t0 }
```

**3. Use `id(42)`:**
```
Lookup id → get scheme ∀?t0. fun(?t0) -> ?t0
Instantiate: replace ?t0 with fresh ?t1
Instance type: fun(?t1) -> ?t1

Call argument: 42 (type Int)
Generate constraint: ?t1 = Int
Unify: ✓ successful
Result: id(42) has type Int
```

**4. Use `id("hello")`:**
```
Lookup id → get same scheme ∀?t0. fun(?t0) -> ?t0
Instantiate: replace ?t0 with fresh ?t2 (different from first use!)
Instance type: fun(?t2) -> ?t2

Call argument: "hello" (type String)
Generate constraint: ?t2 = String
Unify: ✓ successful
Result: id("hello") has type String
```

## Type Annotations and Polymorphism

Explicit type annotations **constrain** but don't prevent polymorphism:

```yolang
let id: fun(Int) -> Int = fun(x) { x };
```

Here, the annotation constrains the binding to monomorphic `Int`:
1. Infer: `fun(?t0) -> ?t0`
2. Annotation: `fun(Int) -> Int`
3. Unify: `?t0 = Int`
4. Generalize: No free variables remain
5. Result: Monomorphic binding (no quantified vars)

## Environment and Free Variables

The **free variable set** is critical for correct generalization:

```yolang
let make_adder = fun(x) {
    fun(y) {
        x + y
    }
};
```

When inferring the inner function:
```
Outer parameter x: type ?t0
Inner parameter y: type ?t1
Body: x + y requires ?t0 = ?t1 (same numeric type)

Free vars in environment: {?t0} (x is bound in outer scope)
All vars in fun(?t1) -> ?t1: just ?t1

Generalize inner function:
  - Free in environment: ?t0
  - Free in inner function: ?t0, ?t1
  - Quantified vars: {?t1}  (NOT ?t0, since it's constrained by x)

Result: ∀?t1. fun(?t1) -> ?t1
  where the ?t0 in the body refers to the outer x
```

This is correct: each call to `make_adder` with a concrete type produces a different adder function for that type, and you can't use the returned function with different types.

## Implementation Details

### Data Structures

```rust
// TypeScheme: represents polymorphic types
pub struct TypeScheme {
    pub quantified_vars: Vec<TypeVar>,  // Bound by ∀
    pub ty: InferType,                  // Body (may contain quantified vars)
}

// InferContext: extended with polymorphic environment
pub struct InferContext {
    env: HashMap<String, InferType>,        // Monomorphic bindings
    poly_env: HashMap<String, TypeScheme>,  // Polymorphic bindings
    // ... other fields
}
```

### Key Methods

**Generalization:**
```rust
// Identify quantified variables based on free vars in environment
pub fn generalize(ty: InferType, free_vars: &HashSet<TypeVar>) -> TypeScheme {
    let quantified_vars = collect_free_vars(&ty)
        .into_iter()
        .filter(|v| !free_vars.contains(v))
        .collect();
    TypeScheme { quantified_vars, ty }
}
```

**Instantiation:**
```rust
// Create fresh type variables for each use
pub fn instantiate(&self, ctx: &mut InferContext) -> InferType {
    let mut subst = Substitution::new();
    for &quantified_var in &self.quantified_vars {
        let fresh = ctx.fresh_var();
        subst.bind(quantified_var, InferType::Var(fresh));
    }
    subst.apply(&self.ty)
}
```

**Lookup with instantiation:**
```rust
pub fn lookup_var(&mut self, name: &str) -> Option<InferType> {
    if let Some(scheme) = self.poly_env.get(name).cloned() {
        return Some(scheme.instantiate(self));  // Fresh vars every time
    }
    self.env.get(name).cloned()
}
```

## Comparison: Monomorphic vs Polymorphic

| Feature | Monomorphic (Rust-style) | Polymorphic (HM-style) |
|---------|--------------------------|------------------------|
| Type parameters | Explicit: `fun id<T>(x: T) -> T` | Implicit: inferred and generalized |
| Binding | `fun id<T>` — generic function | `let id` — polymorphic closure |
| Reuse | Compiler generates specialized versions | Same binding works with all types |
| Code generation | Monomorphization needed | Direct interpretation (no bloat) |
| Error messages | Can be verbose (generic constraints) | Often clearer (concrete unification) |

## Next Steps

To integrate this with the expression inferencer:

1. **Expression walker** generates constraints from AST
2. **Let binding handler:** 
   - Infer RHS type
   - Call `ctx.bind_generalized(name, inferred_type)`
3. **Variable lookup:** Use `ctx.lookup_var(name)` (handles instantiation)
4. **Constraint solving:** Call `solve_constraints()` after full walk

Example pseudocode:
```rust
fn infer_let_binding(name: String, rhs: Expr, ctx: &mut InferContext) {
    let rhs_type = infer_expr(rhs, ctx)?;
    ctx.bind_generalized(name, rhs_type);  // Generalize here
}

fn infer_var(name: String, ctx: &mut InferContext) {
    ctx.lookup_var(&name)  // Instantiate here
}
```

---

**Status:** Type scheme infrastructure complete and ready for integration with constraint generation.
