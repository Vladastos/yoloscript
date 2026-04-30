use std::collections::HashMap;
use crate::ast::{Expr, TypeExpr, Span};
use crate::types::Type;
use crate::error::YolangError;

// ── Type Variables ────────────────────────────────────────────────────────────

/// A type variable used during inference to represent unknown types.
/// Type variables are unified during constraint solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);

impl std::fmt::Display for TypeVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?t{}", self.0)
    }
}

// ── Inference Type ───────────────────────────────────────────────────────────

/// A type used during type inference that may contain type variables.
/// After inference is complete, all type variables should be resolved to concrete Types.
#[derive(Debug, Clone, PartialEq)]
pub enum InferType {
    /// Concrete types that have been resolved
    Concrete(Type),
    /// A type variable (unknown type being inferred)
    Var(TypeVar),
    /// Function type with parameter and return types
    Fun(Vec<InferType>, Box<InferType>),
    /// Tuple type
    Tuple(Vec<InferType>),
    /// Array type
    Array(Box<InferType>),
    /// Named type (struct, enum, etc.)
    Named(String, Vec<InferType>),
}

impl InferType {
    pub fn int() -> Self {
        InferType::Concrete(Type::Int)
    }

    pub fn float() -> Self {
        InferType::Concrete(Type::Float)
    }

    pub fn bool() -> Self {
        InferType::Concrete(Type::Bool)
    }

    pub fn str() -> Self {
        InferType::Concrete(Type::Str)
    }

    pub fn unit() -> Self {
        InferType::Concrete(Type::Unit)
    }

    pub fn var(v: TypeVar) -> Self {
        InferType::Var(v)
    }
}

impl std::fmt::Display for InferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferType::Concrete(t) => write!(f, "{}", t),
            InferType::Var(v) => write!(f, "{}", v),
            InferType::Fun(params, ret) => {
                write!(f, "fun(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            InferType::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            InferType::Array(t) => write!(f, "{}[]", t),
            InferType::Named(name, args) => {
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "<")?;
                    for (i, a) in args.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", a)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }
    }
}

// ── Type Schemes (for let-polymorphism) ──────────────────────────────────────

/// A polymorphic type scheme with universally quantified type variables.
/// Represents types like `∀α. α → α` (the identity function type).
///
/// When a polymorphic binding is created (e.g., `let id = fun(x) { x }`),
/// its inferred type is generalized into a type scheme by identifying
/// which type variables are "free" (not constrained by context).
///
/// Each use of a polymorphic binding instantiates the scheme with fresh
/// type variables, allowing the same binding to work with different concrete types.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeScheme {
    /// Type variables that are universally quantified (bound by ∀)
    pub quantified_vars: Vec<TypeVar>,
    /// The body type (may contain quantified vars and other type vars)
    pub ty: InferType,
}

impl TypeScheme {
    /// Create a type scheme from a type and a set of free type variables.
    /// Variables not in `free_vars` become quantified.
    pub fn generalize(ty: InferType, free_vars: &std::collections::HashSet<TypeVar>) -> Self {
        let quantified_vars = collect_free_vars(&ty)
            .into_iter()
            .filter(|v| !free_vars.contains(v))
            .collect();

        TypeScheme { quantified_vars, ty }
    }

    /// Create a monomorphic scheme (no quantified variables).
    pub fn mono(ty: InferType) -> Self {
        TypeScheme {
            quantified_vars: Vec::new(),
            ty,
        }
    }

    /// Instantiate the scheme with fresh type variables.
    /// Creates a new type where each quantified variable is replaced with
    /// a fresh type variable, allowing the scheme to be used multiple times
    /// with different concrete types.
    pub fn instantiate(&self, ctx: &mut InferContext) -> InferType {
        if self.quantified_vars.is_empty() {
            return self.ty.clone();
        }

        // Create a mapping from quantified vars to fresh vars
        let mut subst = Substitution::new();
        for &quantified_var in &self.quantified_vars {
            let fresh = ctx.fresh_var();
            subst.bind(quantified_var, InferType::Var(fresh));
        }

        // Apply the substitution to instantiate the type
        subst.apply(&self.ty)
    }

    /// Check if this scheme is polymorphic (has quantified variables).
    pub fn is_polymorphic(&self) -> bool {
        !self.quantified_vars.is_empty()
    }

    /// Convert to display form: `∀α. α → α` or just the type if monomorphic.
    pub fn display_with_forall(&self) -> String {
        if self.quantified_vars.is_empty() {
            format!("{}", self.ty)
        } else {
            let mut result = String::from("∀");
            for (i, var) in self.quantified_vars.iter().enumerate() {
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(&format!("{}", var));
            }
            result.push('.');
            result.push(' ');
            result.push_str(&format!("{}", self.ty));
            result
        }
    }
}

impl std::fmt::Display for TypeScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_with_forall())
    }
}

// ── Collecting Free Variables ────────────────────────────────────────────────

/// Collect all type variables that appear in a type.
fn collect_free_vars(ty: &InferType) -> Vec<TypeVar> {
    let mut vars = Vec::new();
    collect_free_vars_impl(ty, &mut vars);
    vars.sort_by_key(|v| v.0);
    vars.dedup();
    vars
}

fn collect_free_vars_impl(ty: &InferType, vars: &mut Vec<TypeVar>) {
    match ty {
        InferType::Var(v) => {
            if !vars.contains(v) {
                vars.push(*v);
            }
        }
        InferType::Fun(params, ret) => {
            for p in params {
                collect_free_vars_impl(p, vars);
            }
            collect_free_vars_impl(ret, vars);
        }
        InferType::Tuple(ts) => {
            for t in ts {
                collect_free_vars_impl(t, vars);
            }
        }
        InferType::Array(t) => collect_free_vars_impl(t, vars),
        InferType::Named(_, args) => {
            for arg in args {
                collect_free_vars_impl(arg, vars);
            }
        }
        InferType::Concrete(_) => {}
    }
}

// ── Constraints ──────────────────────────────────────────────────────────────

/// A constraint stating that two types must unify (be equal).
#[derive(Debug, Clone)]
pub struct Constraint {
    pub lhs: InferType,
    pub rhs: InferType,
    pub span: Span,
}

impl Constraint {
    pub fn new(lhs: InferType, rhs: InferType, span: Span) -> Self {
        Constraint { lhs, rhs, span }
    }
}

// ── Substitution ─────────────────────────────────────────────────────────────

/// A substitution maps type variables to their inferred types.
/// Used to resolve type variables during constraint solving.
#[derive(Debug, Clone)]
pub struct Substitution {
    bindings: HashMap<TypeVar, InferType>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution {
            bindings: HashMap::new(),
        }
    }

    /// Bind a type variable to an inferred type.
    pub fn bind(&mut self, var: TypeVar, ty: InferType) {
        self.bindings.insert(var, ty);
    }

    /// Look up a type variable in the substitution.
    pub fn lookup(&self, var: TypeVar) -> Option<&InferType> {
        self.bindings.get(&var)
    }

    /// Apply the substitution to an inferred type, fully resolving all variables.
    pub fn apply(&self, ty: &InferType) -> InferType {
        match ty {
            InferType::Var(v) => {
                match self.lookup(*v) {
                    Some(resolved) => self.apply(resolved),
                    None => ty.clone(),
                }
            }
            InferType::Fun(params, ret) => {
                let params = params.iter().map(|p| self.apply(p)).collect();
                let ret = Box::new(self.apply(ret));
                InferType::Fun(params, ret)
            }
            InferType::Tuple(ts) => {
                let ts = ts.iter().map(|t| self.apply(t)).collect();
                InferType::Tuple(ts)
            }
            InferType::Array(t) => {
                InferType::Array(Box::new(self.apply(t)))
            }
            InferType::Named(name, args) => {
                let args = args.iter().map(|a| self.apply(a)).collect();
                InferType::Named(name.clone(), args)
            }
            InferType::Concrete(_) => ty.clone(),
        }
    }

    /// Compose two substitutions (apply other, then self).
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = other.clone();
        for (var, ty) in &self.bindings {
            result.bind(*var, self.apply(ty));
        }
        result
    }
}

// ── Type Inference Context ───────────────────────────────────────────────────

/// The inference context maintains state during type inference.
/// It tracks type variables, constraints, polymorphic bindings, and the current environment.
pub struct InferContext {
    /// Counter for generating fresh type variables
    var_counter: u32,
    /// Environment: variable names → inferred types (monomorphic)
    env: HashMap<String, InferType>,
    /// Polymorphic environment: variable names → type schemes (for let-polymorphism)
    poly_env: HashMap<String, TypeScheme>,
    /// Collected constraints to solve
    constraints: Vec<Constraint>,
    /// Current substitution (variable bindings)
    subst: Substitution,
}

impl InferContext {
    pub fn new() -> Self {
        InferContext {
            var_counter: 0,
            env: HashMap::new(),
            poly_env: HashMap::new(),
            constraints: Vec::new(),
            subst: Substitution::new(),
        }
    }

    /// Generate a fresh type variable.
    pub fn fresh_var(&mut self) -> TypeVar {
        let var = TypeVar(self.var_counter);
        self.var_counter += 1;
        var
    }

    /// Add a variable to the monomorphic environment with an inferred type.
    pub fn bind_var(&mut self, name: String, ty: InferType) {
        self.env.insert(name, ty);
    }

    /// Add a variable to the polymorphic environment as a type scheme.
    /// This is used for let-bound functions/closures that should be polymorphic.
    pub fn bind_polymorphic(&mut self, name: String, scheme: TypeScheme) {
        self.poly_env.insert(name, scheme);
    }

    /// Look up a variable, first checking the polymorphic environment, then monomorphic.
    /// If found in polymorphic environment, instantiates the scheme with fresh variables.
    pub fn lookup_var(&mut self, name: &str) -> Option<InferType> {
        // Check polymorphic environment first
        if let Some(scheme) = self.poly_env.get(name).cloned() {
            return Some(scheme.instantiate(self));
        }

        // Fall back to monomorphic environment
        self.env.get(name).cloned()
    }

    /// Look up a variable without instantiation (get the scheme itself).
    /// Used for inspecting polymorphic bindings.
    pub fn lookup_scheme(&self, name: &str) -> Option<&TypeScheme> {
        self.poly_env.get(name)
    }

    /// Generalize a type into a polymorphic scheme and bind it.
    /// This determines which type variables should be quantified based on
    /// what's already in the environment.
    pub fn bind_generalized(&mut self, name: String, ty: InferType) {
        let free_in_env = self.free_vars_in_environment();
        let scheme = TypeScheme::generalize(ty, &free_in_env);
        self.bind_polymorphic(name, scheme);
    }

    /// Collect all free type variables currently in the environment.
    /// These are type variables that are already constrained by context,
    /// so they should NOT be quantified when generalizing new bindings.
    fn free_vars_in_environment(&self) -> std::collections::HashSet<TypeVar> {
        let mut free = std::collections::HashSet::new();

        for ty in self.env.values() {
            for var in collect_free_vars(ty) {
                free.insert(var);
            }
        }

        for scheme in self.poly_env.values() {
            for var in collect_free_vars(&scheme.ty) {
                free.insert(var);
            }
        }

        free
    }

    /// Add a constraint to the constraint set.
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Get all collected constraints.
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    /// Set the substitution (done after solving constraints).
    pub fn set_substitution(&mut self, subst: Substitution) {
        self.subst = subst;
    }

    /// Get the current substitution.
    pub fn substitution(&self) -> &Substitution {
        &self.subst
    }

    /// Apply the current substitution to a type.
    pub fn apply(&self, ty: &InferType) -> InferType {
        self.subst.apply(ty)
    }
}

// ── Unification ──────────────────────────────────────────────────────────────

/// Unify two inferred types. If they can be made equal, returns an updated substitution.
/// If they cannot unify, returns an error.
pub fn unify(ty1: &InferType, ty2: &InferType, subst: &Substitution) -> Result<Substitution, String> {
    let ty1 = subst.apply(ty1);
    let ty2 = subst.apply(ty2);

    match (&ty1, &ty2) {
        // Both concrete types must be identical
        (InferType::Concrete(t1), InferType::Concrete(t2)) => {
            if t1 == t2 {
                Ok(subst.clone())
            } else {
                Err(format!("Type mismatch: {} vs {}", t1, t2))
            }
        }
        // Variable unification
        (InferType::Var(v), t) | (t, InferType::Var(v)) => {
            if let InferType::Var(v2) = t {
                if v == v2 {
                    return Ok(subst.clone());
                }
            }
            if occurs_check(*v, t, subst) {
                Err(format!("Infinite type: {} occurs in {}", v, t))
            } else {
                let mut new_subst = subst.clone();
                new_subst.bind(*v, t.clone());
                Ok(new_subst)
            }
        }
        // Function types
        (InferType::Fun(p1, r1), InferType::Fun(p2, r2)) => {
            if p1.len() != p2.len() {
                return Err(format!(
                    "Function arity mismatch: {} vs {}",
                    p1.len(),
                    p2.len()
                ));
            }
            let mut subst = subst.clone();
            for (a, b) in p1.iter().zip(p2.iter()) {
                subst = unify(a, b, &subst)?;
            }
            unify(r1, r2, &subst)
        }
        // Tuple types
        (InferType::Tuple(ts1), InferType::Tuple(ts2)) => {
            if ts1.len() != ts2.len() {
                return Err(format!(
                    "Tuple length mismatch: {} vs {}",
                    ts1.len(),
                    ts2.len()
                ));
            }
            let mut subst = subst.clone();
            for (a, b) in ts1.iter().zip(ts2.iter()) {
                subst = unify(a, b, &subst)?;
            }
            Ok(subst)
        }
        // Array types
        (InferType::Array(t1), InferType::Array(t2)) => {
            unify(t1, t2, subst)
        }
        // Named types
        (InferType::Named(n1, a1), InferType::Named(n2, a2)) => {
            if n1 != n2 {
                return Err(format!("Type mismatch: {} vs {}", n1, n2));
            }
            if a1.len() != a2.len() {
                return Err(format!(
                    "Type argument count mismatch: {} vs {}",
                    a1.len(),
                    a2.len()
                ));
            }
            let mut subst = subst.clone();
            for (arg1, arg2) in a1.iter().zip(a2.iter()) {
                subst = unify(arg1, arg2, &subst)?;
            }
            Ok(subst)
        }
        // Any other mismatch
        _ => Err(format!("Type mismatch: {} vs {}", ty1, ty2)),
    }
}

/// Check if a type variable occurs in a type (prevents infinite types).
fn occurs_check(var: TypeVar, ty: &InferType, subst: &Substitution) -> bool {
    let ty = subst.apply(ty);
    match ty {
        InferType::Var(v) => v == var,
        InferType::Fun(params, ret) => {
            params.iter().any(|p| occurs_check(var, p, subst))
                || occurs_check(var, &ret, subst)
        }
        InferType::Tuple(ts) => {
            ts.iter().any(|t| occurs_check(var, t, subst))
        }
        InferType::Array(t) => occurs_check(var, &t, subst),
        InferType::Named(_, args) => {
            args.iter().any(|a| occurs_check(var, a, subst))
        }
        InferType::Concrete(_) => false,
    }
}

// ── Public Inference Interface ──────────────────────────────────────────────

/// Solve a set of constraints using unification.
/// Returns a substitution that satisfies all constraints, or an error if no solution exists.
pub fn solve_constraints(constraints: Vec<Constraint>) -> Result<Substitution, YolangError> {
    let mut subst = Substitution::new();

    for constraint in constraints {
        subst = unify(&constraint.lhs, &constraint.rhs, &subst).map_err(|msg| {
            YolangError::TypeError {
                message: format!("{}", msg),
                start: constraint.span.start,
                end: constraint.span.end,
                filename: constraint.span.filename.clone(),
            }
        })?;
    }

    Ok(subst)
}

/// Resolve an inferred type to a concrete type.
/// All type variables must be bound in the substitution.
pub fn resolve_type(ty: &InferType, subst: &Substitution) -> Result<Type, YolangError> {
    let resolved = subst.apply(ty);

    match resolved {
        InferType::Concrete(t) => Ok(t),
        InferType::Var(v) => {
            Err(YolangError::TypeError {
                message: format!("Unresolved type variable: {}", v),
                start: 0,
                end: 0,
                filename: String::from("<internal>"),
            })
        }
        InferType::Fun(params, ret) => {
            let params = params
                .iter()
                .map(|p| resolve_type(p, subst))
                .collect::<Result<_, _>>()?;
            let ret = Box::new(resolve_type(&ret, subst)?);
            Ok(Type::Fun(params, ret))
        }
        InferType::Tuple(ts) => {
            let ts = ts
                .iter()
                .map(|t| resolve_type(t, subst))
                .collect::<Result<_, _>>()?;
            Ok(Type::Tuple(ts))
        }
        InferType::Array(t) => {
            let t = Box::new(resolve_type(&t, subst)?);
            Ok(Type::Array(t))
        }
        InferType::Named(name, args) => {
            let args = args
                .iter()
                .map(|a| resolve_type(a, subst))
                .collect::<Result<_, _>>()?;
            Ok(Type::Named(name, args))
        }
    }
}

// ── Scheme-Aware Operations ──────────────────────────────────────────────────

/// Resolve a type scheme to a concrete type.
/// All type variables (including quantified ones) must be bound.
pub fn resolve_scheme(scheme: &TypeScheme, subst: &Substitution) -> Result<Type, YolangError> {
    resolve_type(&scheme.ty, subst)
}

/// Apply substitution to a type scheme.
/// Substitutes type variables in the body while preserving quantified variables.
pub fn apply_to_scheme(scheme: &TypeScheme, subst: &Substitution) -> TypeScheme {
    TypeScheme {
        quantified_vars: scheme.quantified_vars.clone(),
        ty: subst.apply(&scheme.ty),
    }
}
