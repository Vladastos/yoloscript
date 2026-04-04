use pest::Parser;
use pest_derive::Parser;

use crate::ast::Program;
use crate::error::YolangError;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct YolangParser;

/// Parse a Yolang source string into an untyped AST.
pub fn parse(source: &str, filename: &str) -> Result<Program, YolangError> {
    let pairs = YolangParser::parse(Rule::program, source).map_err(|e| {
        // pest errors carry position information
        let (start, end) = match e.location {
            pest::error::InputLocation::Pos(p) => (p, p),
            pest::error::InputLocation::Span((s, e)) => (s, e),
        };
        YolangError::ParseError {
            message: e.variant.to_string(),
            start,
            end,
            filename: filename.to_string(),
        }
    })?;

    build_ast(pairs, filename)
}

fn build_ast(
    pairs: pest::iterators::Pairs<Rule>,
    _filename: &str,
) -> Result<Program, YolangError> {
    // TODO: implement AST building from pest pairs
    // Each Rule variant maps to one of the AST node types in crate::ast.
    // This is intentionally left as a stub — the grammar.pest file must be
    // written first, then this function is filled in rule by rule.
    let _ = pairs;
    Ok(vec![])
}
