use crate::error::Span;
use crate::types::Type;

/// A typed program is a list of typed declarations.
pub type TypedProgram = Vec<TypedDecl>;

/// Mirrors `ast::Decl` but every expression node carries a resolved `Type`.
/// Generic declarations do not appear here — they have been monomorphised
/// into concrete instantiations by the type checker.
#[derive(Debug, Clone)]
pub enum TypedDecl {
    // TODO: mirror ast::Decl with Type annotations on all expression nodes
    // Stub to allow the project to compile while the type checker is being built.
    _Placeholder(Span, Type),
}
