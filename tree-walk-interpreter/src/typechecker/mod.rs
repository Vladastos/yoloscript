use crate::ast::Program;
use crate::error::YolangError;
use crate::typed_ast::TypedProgram;

/// Run the type checker over an untyped AST, producing a fully typed AST.
/// All generic instantiations are monomorphised here.
pub fn check(program: Program) -> Result<TypedProgram, YolangError> {
    // TODO: implement type checker
    // Phases (in order):
    //   1. Collect all top-level declarations into a declaration table
    //   2. Resolve types in all declarations (struct fields, enum variants, trait signatures)
    //   3. Type-check and infer types for function bodies
    //   4. Monomorphise generic instantiations
    //   5. Check match exhaustiveness
    let _ = program;
    Ok(vec![])
}
