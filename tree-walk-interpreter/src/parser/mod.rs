use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;
use crate::error::{Span, YolangError};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct YolangParser;

/// Parse a Yolang source string into an untyped AST.
pub fn parse(source: &str, filename: &str) -> Result<Program, YolangError> {
    let pairs = YolangParser::parse(Rule::program, source).map_err(|e| {
        let (start, end) = match e.location {
            pest::error::InputLocation::Pos(p) => (p, p),
            pest::error::InputLocation::Span((s, e)) => (s, e),
        };
        YolangError::ParseErrorWithLine {
            message: e.variant.to_string(),
            start,
            end,
            line : e.line().to_string(),
            filename: filename.to_string(),
        }
    })?;

    build_ast(pairs, filename)
}

// ── Helper functions for spans and pair extraction ──────────────────────────

fn span_of(pair: &Pair<Rule>, filename: &str) -> Span {
    let span = pair.as_span();
    Span::new(span.start(), span.end(), filename)
}

fn next_non_whitespace<'a>(pairs: &mut Pairs<'a, Rule>) -> Option<Pair<'a, Rule>> {
    pairs.find(|p| !matches!(p.as_rule(), Rule::WHITESPACE))
}

// ── Top-level parsing ───────────────────────────────────────────────────────

fn build_ast(pairs: Pairs<Rule>, filename: &str) -> Result<Program, YolangError> {
    let mut program = vec![];
    let mut inner = pairs.into_iter();

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::decl => {
                program.push(parse_decl(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    Ok(program)
}

// ── Declaration parsing ────────────────────────────────────────────────────

fn parse_decl(mut pairs: Pairs<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let pair = pairs.next().ok_or_else(|| {
        YolangError::ParseError {
            message: "Empty decl".to_string(),
            start: 0,
            end: 0,
            filename: filename.to_string(),
        }
    })?;

    match pair.as_rule() {
        Rule::let_decl => parse_let_decl(pair, filename),
        Rule::mut_decl => parse_mut_decl(pair, filename),
        Rule::fun_decl => parse_fun_decl(pair, filename),
        Rule::struct_decl => parse_struct_decl(pair, filename),
        Rule::enum_decl => parse_enum_decl(pair, filename),
        Rule::impl_block => parse_impl_block(pair, filename),
        Rule::trait_decl => parse_trait_decl(pair, filename),
        Rule::stmt => {
            let stmt = parse_stmt(pair.into_inner(), filename)?;
            Ok(Decl::Stmt(stmt))
        }
        _ => Err(YolangError::ParseError {
            message: format!("Unexpected decl rule: {:?}", pair.as_rule()),
            start: pair.as_span().start(),
            end: pair.as_span().end(),
            filename: filename.to_string(),
        }),
    }
}

fn parse_let_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing name in let", &span))?
        .as_str()
        .to_string();

    let mut type_ann = None;
    let mut value = None;

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::type_expr => {
                type_ann = Some(parse_type_expr(pair.into_inner(), filename)?);
            }
            Rule::expr => {
                value = Some(parse_expr(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    let value = value.ok_or_else(|| YolangError::parse("Missing value in let", &span))?;
    Ok(Decl::Let(LetDecl { name, type_ann, value, span }))
}

fn parse_mut_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing name in mut", &span))?
        .as_str()
        .to_string();

    let mut type_ann = None;
    let mut value = None;

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::type_expr => {
                type_ann = Some(parse_type_expr(pair.into_inner(), filename)?);
            }
            Rule::expr => {
                value = Some(parse_expr(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    let value = value.ok_or_else(|| YolangError::parse("Missing value in mut", &span))?;
    Ok(Decl::Mut(MutDecl { name, type_ann, value, span }))
}

fn parse_fun_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing function name", &span))?
        .as_str()
        .to_string();

    let mut generics = vec![];
    let mut params = vec![];
    let mut return_type = None;
    let mut body = vec![];

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::generic_params => {
                generics = parse_generic_params(pair.into_inner(), filename)?;
            }
            Rule::param_list => {
                params = parse_param_list(pair.into_inner(), filename)?;
            }
            Rule::type_expr => {
                return_type = Some(parse_type_expr(pair.into_inner(), filename)?);
            }
            Rule::block => {
                body = parse_block(pair.into_inner(), filename)?;
            }
            _ => {}
        }
    }

    Ok(Decl::Fun(FunDecl { name, generics, params, return_type, body, span }))
}

fn parse_struct_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing struct name", &span))?
        .as_str()
        .to_string();

    let mut generics = vec![];
    let mut fields = vec![];

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::generic_params => {
                generics = parse_generic_params(pair.into_inner(), filename)?;
            }
            Rule::struct_fields => {
                fields = parse_struct_fields(pair.into_inner(), filename)?;
            }
            _ => {}
        }
    }

    Ok(Decl::Struct(StructDecl { name, generics, fields, span }))
}

fn parse_enum_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing enum name", &span))?
        .as_str()
        .to_string();

    let mut generics = vec![];
    let mut variants = vec![];

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::generic_params => {
                generics = parse_generic_params(pair.into_inner(), filename)?;
            }
            Rule::enum_variants => {
                variants = parse_enum_variants(pair.into_inner(), filename)?;
            }
            _ => {}
        }
    }

    Ok(Decl::Enum(EnumDecl { name, generics, variants, span }))
}

fn parse_impl_block(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let mut trait_name = None;
    let mut target_type = None;
    let mut methods = vec![];

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::type_path => {
                // First type_path is either trait name or target type
                let path_vec = parse_type_path(pair.into_inner());
                if let Some(next) = inner.clone().next() {
                    // If there's a `for`, this type_path is the trait
                    if matches!(next.as_rule(), Rule::type_expr) {
                        trait_name = Some(path_vec.join("::"));
                    }
                } else {
                    // Only one type_path, this is the target
                    target_type = Some(TypeExpr::Named(path_vec.join("::"), vec![]));
                }
            }
            Rule::type_expr => {
                if target_type.is_none() {
                    target_type = Some(parse_type_expr(pair.into_inner(), filename)?);
                }
            }
            Rule::fun_decl => {
                if let Decl::Fun(fd) = parse_fun_decl(pair, filename)? {
                    methods.push(fd);
                }
            }
            _ => {}
        }
    }

    let target_type = target_type.ok_or_else(|| YolangError::parse("Missing target type in impl", &span))?;
    Ok(Decl::Impl(ImplBlock { trait_name, target_type, methods, span }))
}

fn parse_trait_decl(pair: Pair<Rule>, filename: &str) -> Result<Decl, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or_else(|| YolangError::parse("Missing trait name", &span))?
        .as_str()
        .to_string();

    let mut methods = vec![];

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::trait_method => {
                methods.push(parse_trait_method(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    Ok(Decl::Trait(TraitDecl { name, methods, span }))
}

// ── Parameter parsing ───────────────────────────────────────────────────────

fn parse_generic_params(pairs: Pairs<Rule>, filename: &str) -> Result<Vec<GenericParam>, YolangError> {
    let mut params = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::generic_param => {
                let span = span_of(&pair, filename);
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or_else(|| YolangError::parse("Missing generic param name", &span))?.as_str().to_string();
                let bound = if let Some(p) = inner.next() {
                    Some(parse_type_expr(p.into_inner(), filename)?)
                } else {
                    None
                };
                params.push(GenericParam { name, bound });
            }
            _ => {}
        }
    }

    Ok(params)
}

fn parse_param_list(pairs: Pairs<Rule>, filename: &str) -> Result<Vec<Param>, YolangError> {
    let mut params = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::param => {
                params.push(parse_param(pair, filename)?);
            }
            _ => {}
        }
    }

    Ok(params)
}

fn parse_param(pair: Pair<Rule>, filename: &str) -> Result<Param, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| YolangError::parse("Empty param", &span))?;

    if first.as_rule() == Rule::ident {
        let name = first.as_str().to_string();
        let type_ann = if let Some(p) = inner.next() {
            parse_type_expr(p.into_inner(), filename)?
        } else {
            TypeExpr::Named(name.clone(), vec![])
        };
        Ok(Param { mutable: false, name, type_ann, span })
    } else {
        // "self" or "mut self"
        let mutable = first.as_str() == "mut";
        let type_ann = TypeExpr::Named("Self".to_string(), vec![]);
        Ok(Param { mutable, name: "self".to_string(), type_ann, span })
    }
}

fn parse_struct_fields(pairs: Pairs<Rule>, filename: &str) -> Result<Vec<FieldDef>, YolangError> {
    let mut fields = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::struct_field => {
                let span = span_of(&pair, filename);
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or_else(|| YolangError::parse("Missing field name", &span))?.as_str().to_string();
                let type_ann = inner.next().map(|p| parse_type_expr(p.into_inner(), filename)).transpose()?;
                fields.push(FieldDef { name, type_ann, span });
            }
            _ => {}
        }
    }

    Ok(fields)
}

fn parse_enum_variants(pairs: Pairs<Rule>, filename: &str) -> Result<Vec<VariantDef>, YolangError> {
    let mut variants = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::enum_variant => {
                let span = span_of(&pair, filename);
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or_else(|| YolangError::parse("Missing variant name", &span))?.as_str().to_string();
                let fields = if let Some(p) = inner.next() {
                    parse_struct_fields(p.into_inner(), filename)?
                } else {
                    vec![]
                };
                variants.push(VariantDef { name, fields, span });
            }
            _ => {}
        }
    }

    Ok(variants)
}

fn parse_trait_method(mut pairs: Pairs<Rule>, filename: &str) -> Result<TraitMethod, YolangError> {
    let name = pairs.next().ok_or_else(|| YolangError::parse("Missing trait method name", &Span::new(0, 0, filename)))?.as_str().to_string();

    let mut generics = vec![];
    let mut params = vec![];
    let mut return_type = None;
    let mut default_body = None;

    let span = Span::new(0, 1, filename); // Simplified for now

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::generic_params => {
                generics = parse_generic_params(pair.into_inner(), filename)?;
            }
            Rule::param_list => {
                params = parse_param_list(pair.into_inner(), filename)?;
            }
            Rule::type_expr => {
                return_type = Some(parse_type_expr(pair.into_inner(), filename)?);
            }
            Rule::block => {
                default_body = Some(parse_block(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    Ok(TraitMethod { name, generics, params, return_type, default_body, span })
}

// ── Block and statement parsing ─────────────────────────────────────────────

fn parse_block(pairs: Pairs<Rule>, filename: &str) -> Result<Block, YolangError> {
    let mut block = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::decl => {
                block.push(parse_decl(pair.into_inner(), filename)?);
            }
            _ => {}
        }
    }

    Ok(block)
}

fn parse_stmt(mut pairs: Pairs<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let pair = pairs.next().ok_or_else(|| {
        YolangError::ParseError {
            message: "Empty stmt".to_string(),
            start: 0,
            end: 0,
            filename: filename.to_string(),
        }
    })?;

    match pair.as_rule() {
        Rule::if_stmt => parse_if_stmt(pair, filename),
        Rule::while_stmt => parse_while_stmt(pair, filename),
        Rule::for_stmt => parse_for_stmt(pair, filename),
        Rule::for_in_stmt => parse_for_in_stmt(pair, filename),
        Rule::loop_stmt => parse_loop_stmt(pair, filename),
        Rule::match_stmt => Ok(Stmt::Expr(parse_match_expr(pair, filename)?)),
        Rule::return_stmt => parse_return_stmt(pair, filename),
        Rule::break_stmt => parse_break_stmt(pair, filename),
        Rule::continue_stmt => {
            let span = span_of(&pair, filename);
            Ok(Stmt::Continue(span))
        }
        Rule::expr_stmt => {
            let span = span_of(&pair, filename);
            let mut inner = pair.into_inner();
            let expr = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Empty expr_stmt", &span))?.into_inner(), filename)?;
            Ok(Stmt::Expr(expr))
        }
        _ => Err(YolangError::ParseError {
            message: format!("Unexpected stmt rule: {:?}", pair.as_rule()),
            start: pair.as_span().start(),
            end: pair.as_span().end(),
            filename: filename.to_string(),
        }),
    }
}

fn parse_match_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();
    inner.next(); // skip "match"
    let scrutinee = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing match scrutinee", &span))?.into_inner(), filename)?;

    let mut arms = vec![];
    for p in inner {
        if matches!(p.as_rule(), Rule::match_arm) {
            arms.push(parse_match_arm(p, filename)?);
        }
    }
    Ok(Expr::Match { scrutinee: Box::new(scrutinee), arms, span })
}

fn parse_if_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let condition = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing if condition", &span))?.into_inner(), filename)?;
    let then_branch = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing then branch", &span))?.into_inner(), filename)?;

    let else_branch = if let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::if_stmt => {
                if let Stmt::If(if_stmt) = parse_if_stmt(pair, filename)? {
                    Some(ElseBranch::If(Box::new(if_stmt)))
                } else {
                    None
                }
            }
            Rule::block => Some(ElseBranch::Block(parse_block(pair.into_inner(), filename)?)),
            _ => None,
        }
    } else {
        None
    };

    Ok(Stmt::If(IfStmt { condition, then_branch, else_branch, span }))
}

fn parse_while_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let condition = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing while condition", &span))?.into_inner(), filename)?;
    let body = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing while body", &span))?.into_inner(), filename)?;

    Ok(Stmt::While(WhileStmt { condition, body, span }))
}

fn parse_for_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    // First is for_init
    let init_pair = inner.next().ok_or_else(|| YolangError::parse("Missing for_init", &span))?;
    let init = if matches!(init_pair.as_rule(), Rule::for_init) {
        let mut init_inner = init_pair.into_inner();
        if let Some(init_inner_pair) = init_inner.next() {
            match init_inner_pair.as_rule() {
                Rule::mut_decl => Some(Box::new(parse_mut_decl(init_inner_pair, filename)?)),
                Rule::expr_stmt => {
                    let mut expr_inner = init_inner_pair.into_inner();
                    if let Some(e) = expr_inner.next() {
                        let expr = parse_expr(e.into_inner(), filename)?;
                        Some(Box::new(Decl::Stmt(Stmt::Expr(expr))))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    // Next two are condition and step expressions
    let condition = if let Some(p) = inner.next() {
        if matches!(p.as_rule(), Rule::expr) {
            Some(parse_expr(p.into_inner(), filename)?)
        } else {
            None
        }
    } else {
        None
    };

    let step = if let Some(p) = inner.next() {
        if matches!(p.as_rule(), Rule::expr) {
            Some(parse_expr(p.into_inner(), filename)?)
        } else {
            None
        }
    } else {
        None
    };

    // Finally, the body
    let body = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing for body", &span))?.into_inner(), filename)?;

    Ok(Stmt::For(ForStmt { init, condition, step, body, span }))
}

fn parse_for_in_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let binding = inner.next().ok_or_else(|| YolangError::parse("Missing for-in binding", &span))?.as_str().to_string();
    let iterable = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing for-in iterable", &span))?.into_inner(), filename)?;
    let body = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing for-in body", &span))?.into_inner(), filename)?;

    Ok(Stmt::ForIn(ForInStmt { binding, iterable, body, span }))
}

fn parse_loop_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let body = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing loop body", &span))?.into_inner(), filename)?;

    Ok(Stmt::Loop(LoopStmt { body, span }))
}

fn parse_return_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let value = if let Some(p) = inner.next() {
        Some(parse_expr(p.into_inner(), filename)?)
    } else {
        None
    };

    Ok(Stmt::Return(ReturnStmt { value, span }))
}

fn parse_break_stmt(pair: Pair<Rule>, filename: &str) -> Result<Stmt, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let value = if let Some(p) = inner.next() {
        Some(parse_expr(p.into_inner(), filename)?)
    } else {
        None
    };

    Ok(Stmt::Break(BreakStmt { value, span }))
}

// ── Expression parsing ──────────────────────────────────────────────────────

fn parse_expr(mut pairs: Pairs<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let pair = pairs.next().ok_or_else(|| {
        YolangError::ParseError {
            message: "Empty expression".to_string(),
            start: 0,
            end: 0,
            filename: filename.to_string(),
        }
    })?;

    parse_expr_from_pair(pair, filename)
}

fn parse_expr_from_pair(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let rule = pair.as_rule();
    let span = span_of(&pair, filename);
    let str_value = pair.as_str().to_string();

    match rule {
        // Primary expressions
        Rule::int_lit => {
            let value = str_value.replace("_", "").parse::<i64>()
                .map_err(|_| YolangError::parse("Invalid integer literal", &span))?;
            Ok(Expr::Literal(Literal::Int(value), span))
        }
        Rule::float_lit => {
            let value = str_value.parse::<f64>()
                .map_err(|_| YolangError::parse("Invalid float literal", &span))?;
            Ok(Expr::Literal(Literal::Float(value), span))
        }
        Rule::string_lit => {
            let raw = &str_value;
            let unquoted = &raw[1..raw.len() - 1];
            let unescaped = unescape_string(unquoted);
            Ok(Expr::Literal(Literal::Str(unescaped), span))
        }
        Rule::bool_lit => {
            let value = str_value == "true";
            Ok(Expr::Literal(Literal::Bool(value), span))
        }
        Rule::nope_lit => Ok(Expr::Literal(Literal::Nope, span)),
        Rule::unit_lit => Ok(Expr::Literal(Literal::Unit, span)),

        // Path/identifier
        Rule::path_expr => {
            let inner = pair.into_inner();
            let parts = parse_type_path(inner);
            if parts.len() == 1 {
                Ok(Expr::Ident(parts[0].clone(), span))
            } else {
                Ok(Expr::Path(parts, span))
            }
        }

        // Collections
        Rule::tuple_or_paren => {
            let mut inner = pair.into_inner();
            let first = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Empty tuple", &span))?.into_inner(), filename)?;

            // Check if there are more expressions (making it a tuple)
            let remaining: Vec<_> = inner.map(|p| parse_expr(p.into_inner(), filename)).collect::<Result<_, _>>()?;

            if remaining.is_empty() {
                Ok(first)
            } else {
                let mut elems = vec![first];
                elems.extend(remaining);
                Ok(Expr::Tuple(elems, span))
            }
        }

        Rule::array_lit => {
            let mut elems = vec![];
            for p in pair.into_inner() {
                if matches!(p.as_rule(), Rule::expr) {
                    elems.push(parse_expr(p.into_inner(), filename)?);
                }
            }
            Ok(Expr::Array(elems, span))
        }

        // Control flow expressions
        Rule::if_expr => {
            let mut inner = pair.into_inner();
            let condition = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing if condition", &span))?.into_inner(), filename)?;
            let then_branch = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing then block", &span))?.into_inner(), filename)?;
            let else_branch = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing else block", &span))?.into_inner(), filename)?;
            Ok(Expr::If { condition: Box::new(condition), then_branch, else_branch, span })
        }

        Rule::loop_expr => {
            let mut inner = pair.into_inner();
            let body = parse_block(inner.next().ok_or_else(|| YolangError::parse("Missing loop body", &span))?.into_inner(), filename)?;
            Ok(Expr::Loop { body, span })
        }

        Rule::match_expr => parse_match_expr(pair, filename),

        // Closures
        Rule::closure_expr => {
            let mut inner = pair.into_inner();
            let mut params = vec![];
            let mut return_type = None;
            let mut body = vec![];

            while let Some(p) = inner.next() {
                match p.as_rule() {
                    Rule::param_list => {
                        params = parse_param_list(p.into_inner(), filename)?;
                    }
                    Rule::type_expr => {
                        return_type = Some(parse_type_expr(p.into_inner(), filename)?);
                    }
                    Rule::block => {
                        body = parse_block(p.into_inner(), filename)?;
                    }
                    _ => {}
                }
            }

            Ok(Expr::Closure { params, return_type, body, span })
        }

        // Struct literals
        Rule::struct_literal => {
            let mut inner = pair.into_inner();
            let path_pair = inner.next().ok_or_else(|| YolangError::parse("Missing struct name", &span))?;
            let path = parse_type_path(path_pair.into_inner());

            let mut fields = vec![];
            for p in inner {
                if matches!(p.as_rule(), Rule::field_init) {
                    let mut field_inner = p.into_inner();
                    let name = field_inner.next().ok_or_else(|| YolangError::parse("Missing field name", &span))?.as_str().to_string();
                    let value = parse_expr(field_inner.next().ok_or_else(|| YolangError::parse("Missing field value", &span))?.into_inner(), filename)?;
                    fields.push((name, value));
                }
            }
            Ok(Expr::StructLiteral { path, fields, span })
        }

        // Expression rules with operators/nesting
        Rule::assign_expr => parse_assign_expr(pair, filename),
        Rule::or_expr => parse_binary_expr(pair, filename, parse_and_expr_item),
        Rule::and_expr => parse_binary_expr(pair, filename, parse_cmp_expr_item),
        Rule::cmp_expr => parse_cmp_expr(pair, filename),
        Rule::range_expr => parse_range_expr(pair, filename),
        Rule::add_expr => parse_binary_expr(pair, filename, parse_mul_expr_item),
        Rule::mul_expr => parse_binary_expr(pair, filename, parse_cast_expr_item),
        Rule::cast_expr => parse_cast_expr(pair, filename),
        Rule::unary_expr => parse_unary_expr(pair, filename),
        Rule::postfix_expr => parse_postfix_expr(pair, filename),
        Rule::primary_expr => {
            let mut inner = pair.into_inner();
            // delegate to the contained rule
            if let Some(p) = inner.next() {
                parse_expr_from_pair(p, filename)
            } else {
                Err(YolangError::parse("Empty primary_expr", &span))
            }
        }

        _ => Err(YolangError::ParseError {
            message: format!("Unexpected expr rule: {:?}", rule),
            start: pair.as_span().start(),
            end: pair.as_span().end(),
            filename: filename.to_string(),
        }),
    }
}

fn parse_assign_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let first = inner.next().ok_or_else(|| YolangError::parse("Empty assign expr", &span))?;
    let mut expr = parse_expr_from_pair(first, filename)?;

    // Check for assignment operator
    if let Some(op_pair) = inner.next() {
        if matches!(op_pair.as_rule(), Rule::assign_op) {
            let op = parse_assign_op(op_pair.as_str());
            let rhs = parse_expr_from_pair(inner.next().ok_or_else(|| YolangError::parse("Missing RHS for assignment", &span))?, filename)?;
            let target = expr_to_assign_target(&expr, filename)?;
            expr = Expr::Assign(target, op, Box::new(rhs), span);
        } else {
            // No assignment, must be or_expr
            expr = fold_binary_expr(expr, op_pair, inner, filename)?;
        }
    }

    Ok(expr)
}

fn parse_binary_expr<F>(pair: Pair<Rule>, filename: &str, mut parse_next: F) -> Result<Expr, YolangError>
where
    F: FnMut(Pair<Rule>, &str) -> Result<Expr, YolangError> + Copy,
{
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| YolangError::parse("Empty binary expr", &span))?;
    let mut expr = parse_next(first, filename)?;

    // Iterate through operator-operand pairs
    while let Some(op_pair) = inner.next() {
        let op = parse_binary_op(&op_pair);
        let operand = inner.next().ok_or_else(|| YolangError::parse("Missing operand", &span))?;
        let rhs = parse_next(operand, filename)?;
        let op_span = span_of(&op_pair, filename);
        expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs), op_span);
    }

    Ok(expr)
}

fn parse_and_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::and_expr) {
        parse_binary_expr(pair, filename, parse_cmp_expr_item)
    } else {
        parse_cmp_expr_item(pair, filename)
    }
}

fn parse_cmp_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::cmp_expr) {
        parse_cmp_expr(pair, filename)
    } else {
        parse_range_expr_item(pair, filename)
    }
}

fn parse_range_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::range_expr) {
        parse_range_expr(pair, filename)
    } else {
        parse_add_expr_item(pair, filename)
    }
}

fn parse_add_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::add_expr) {
        parse_binary_expr(pair, filename, parse_mul_expr_item)
    } else {
        parse_mul_expr_item(pair, filename)
    }
}

fn parse_mul_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::mul_expr) {
        parse_binary_expr(pair, filename, parse_cast_expr_item)
    } else {
        parse_cast_expr_item(pair, filename)
    }
}

fn parse_cast_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::cast_expr) {
        parse_cast_expr(pair, filename)
    } else {
        parse_unary_expr_item(pair, filename)
    }
}

fn parse_unary_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::unary_expr) {
        parse_unary_expr(pair, filename)
    } else {
        parse_postfix_expr_item(pair, filename)
    }
}

fn parse_postfix_expr_item(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    if matches!(pair.as_rule(), Rule::postfix_expr) {
        parse_postfix_expr(pair, filename)
    } else {
        parse_expr_from_pair(pair, filename)
    }
}

fn parse_cmp_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let left = parse_range_expr_item(inner.next().ok_or_else(|| YolangError::parse("Empty cmp expr", &span))?, filename)?;

    if let Some(op_pair) = inner.next() {
        if matches!(op_pair.as_rule(), Rule::cmp_op) {
            let op = parse_binary_op(&op_pair);
            let right = parse_range_expr_item(inner.next().ok_or_else(|| YolangError::parse("Missing RHS for comparison", &span))?, filename)?;
            Ok(Expr::BinOp(Box::new(left), op, Box::new(right), span))
        } else {
            // Just a range_expr, parse it recursively
            fold_binary_expr(left, op_pair, inner, filename)
        }
    } else {
        Ok(left)
    }
}

fn parse_range_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let left = parse_add_expr_item(inner.next().ok_or_else(|| YolangError::parse("Empty range expr", &span))?, filename)?;

    if let Some(op_pair) = inner.next() {
        if matches!(op_pair.as_rule(), Rule::range_op) {
            let op = parse_binary_op(&op_pair);
            let right = parse_add_expr_item(inner.next().ok_or_else(|| YolangError::parse("Missing RHS for range", &span))?, filename)?;
            Ok(Expr::BinOp(Box::new(left), op, Box::new(right), span))
        } else {
            fold_binary_expr(left, op_pair, inner, filename)
        }
    } else {
        Ok(left)
    }
}

fn parse_cast_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let mut expr = parse_unary_expr_item(inner.next().ok_or_else(|| YolangError::parse("Empty cast expr", &span))?, filename)?;

    while let Some(p) = inner.next() {
        if matches!(p.as_rule(), Rule::type_expr) {
            let target_type = parse_type_expr(p.into_inner(), filename)?;
            expr = Expr::Cast { expr: Box::new(expr), target_type, span: span.clone() };
        }
    }

    Ok(expr)
}

fn parse_unary_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let first = inner.next().ok_or_else(|| YolangError::parse("Empty unary expr", &span))?;
    let first_str = first.as_str();

    if first_str == "!" || first_str == "-" {
        let op = parse_unary_op(first_str);
        let expr = parse_unary_expr_item(inner.next().ok_or_else(|| YolangError::parse("Missing operand for unary", &span))?, filename)?;
        Ok(Expr::UnaryOp(op, Box::new(expr), span))
    } else {
        parse_postfix_expr_item(first, filename)
    }
}

fn parse_postfix_expr(pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let primary = inner.next().ok_or_else(|| YolangError::parse("Empty postfix expr", &span))?;
    let mut expr = parse_expr_from_pair(primary, filename)?;

    for postfix_pair in inner {
        if matches!(postfix_pair.as_rule(), Rule::postfix) {
            expr = parse_postfix_op(expr, postfix_pair, filename)?;
        }
    }

    Ok(expr)
}

fn parse_postfix_op(base: Expr, pair: Pair<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let span = span_of(&pair, filename);
    let pair_str = pair.as_str();
    let mut inner = pair.into_inner();

    if pair_str.starts_with('(') {
        // Function call
        let mut args = vec![];
        for p in inner {
            if matches!(p.as_rule(), Rule::expr) {
                args.push(parse_expr(p.into_inner(), filename)?);
            }
        }
        Ok(Expr::Call { callee: Box::new(base), args, span })
    } else if pair_str.starts_with('[') {
        // Index
        let index_expr = parse_expr(inner.next().ok_or_else(|| YolangError::parse("Missing index", &span))?.into_inner(), filename)?;
        Ok(Expr::Index { object: Box::new(base), index: Box::new(index_expr), span })
    } else if pair_str == "?" {
        // Error propagation
        Ok(Expr::PropagateError { expr: Box::new(base), span })
    } else if let Some(field_part) = pair_str.strip_prefix('.') {
        // Field access, method call, or tuple access
        if field_part.chars().all(|c| c.is_ascii_digit()) {
            // Tuple access
            let idx = field_part.parse::<usize>()
                .map_err(|_| YolangError::parse("Invalid tuple index", &span))?;
            Ok(Expr::TupleAccess { object: Box::new(base), index: idx, span })
        } else if field_part.contains('(') {
            // Method call
            let method_name = field_part.split('(').next().unwrap_or(field_part).to_string();
            let mut args = vec![];
            for p in inner {
                if matches!(p.as_rule(), Rule::expr) {
                    args.push(parse_expr(p.into_inner(), filename)?);
                }
            }
            Ok(Expr::MethodCall {
                receiver: Box::new(base),
                method: method_name,
                args,
                span,
            })
        } else {
            // Field access
            Ok(Expr::FieldAccess { object: Box::new(base), field: field_part.to_string(), span })
        }
    } else {
        Err(YolangError::parse("Unknown postfix operator", &span))
    }
}

fn fold_binary_expr(mut expr: Expr, op_pair: Pair<Rule>, mut remaining: Pairs<Rule>, filename: &str) -> Result<Expr, YolangError> {
    let op = parse_binary_op(&op_pair);
    let op_span = span_of(&op_pair, filename);
    let rhs = parse_expr_from_pair(remaining.next().ok_or_else(|| YolangError::parse("Missing RHS", &op_span))?, filename)?;
    expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs), op_span);
    Ok(expr)
}

fn parse_match_arm(pair: Pair<Rule>, filename: &str) -> Result<MatchArm, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();

    let pattern = parse_pattern(inner.next().ok_or_else(|| YolangError::parse("Missing match pattern", &span))?, filename)?;
    
    let mut guard = None;
    let mut body = None;

    while let Some(p) = inner.next() {
        match p.as_rule() {
            Rule::expr => {
                // Could be either guard or body
                // If there's another expr after this, this is the guard
                let remaining_count = inner.clone().count();
                if remaining_count > 0 {
                    guard = Some(parse_expr(p.into_inner(), filename)?);
                } else {
                    body = Some(parse_expr(p.into_inner(), filename)?);
                }
            }
            _ => {}
        }
    }

    let body = body.ok_or_else(|| YolangError::parse("Missing match arm body", &span))?;
    Ok(MatchArm { pattern, guard, body, span })
}

// ── Pattern parsing ────────────────────────────────────────────────────────

fn parse_pattern(pair: Pair<Rule>, filename: &str) -> Result<Pattern, YolangError> {
    let span = span_of(&pair, filename);

    match pair.as_rule() {
        Rule::pattern => {
            let mut inner = pair.into_inner();
            if let Some(p) = inner.next() {
                parse_pattern(p, filename)
            } else {
                Err(YolangError::parse("Empty pattern", &span))
            }
        }
        _ => {
            let text = pair.as_str().trim();
            if text == "_" {
                Ok(Pattern::Wildcard(span))
            } else if text == "nope" {
                Ok(Pattern::Nope(span))
            } else if let Ok(value) = text.replace("_", "").parse::<i64>() {
                Ok(Pattern::Literal(Literal::Int(value), span))
            } else if text == "true" || text == "false" {
                let value = text == "true";
                Ok(Pattern::Literal(Literal::Bool(value), span))
            } else if text.starts_with('"') && text.ends_with('"') {
                let unquoted = &text[1..text.len() - 1];
                Ok(Pattern::Literal(Literal::Str(unescape_string(unquoted)), span))
            } else if text.contains(" { ") {
                let parts: Vec<&str> = text.splitn(2, " { ").collect();
                let path_str = parts[0];
                let path = path_str.split("::").map(|s| s.to_string()).collect();
                let fields_str = parts.get(1).copied().unwrap_or("").trim();
                let fields_str = if fields_str.ends_with('}') { &fields_str[..fields_str.len() - 1] } else { fields_str };
                let field_names = if fields_str.is_empty() { vec![] } else { fields_str.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>() };
                let fields = field_names;
                Ok(Pattern::EnumVariant { path, fields, span })
            } else if text.contains("::") {
                let parts = text.split("::").map(|s| s.to_string()).collect();
                Ok(Pattern::EnumVariant { path: parts, fields: vec![], span })
            } else {
                Ok(Pattern::Binding(text.to_string(), span))
            }
        }
    }
}

// ── Type expression parsing ────────────────────────────────────────────────

fn parse_type_expr(mut pairs: Pairs<Rule>, filename: &str) -> Result<TypeExpr, YolangError> {
    let span = Span::new(0, 1, filename);

    let pair = pairs.next().ok_or_else(|| YolangError::parse("Empty type expression", &span))?;

    match pair.as_rule() {
        Rule::unit_type => Ok(TypeExpr::Unit),
        Rule::tuple_type => {
            let mut elems = vec![];
            for p in pair.into_inner() {
                if matches!(p.as_rule(), Rule::type_expr) {
                    elems.push(parse_type_expr(p.into_inner(), filename)?);
                }
            }
            Ok(TypeExpr::Tuple(elems))
        }
        Rule::fun_type => {
            let mut inner = pair.into_inner();
            let mut params = vec![];
            let mut return_type = Box::new(TypeExpr::Unit);

            while let Some(p) = inner.next() {
                match p.as_rule() {
                    Rule::type_list => {
                        for tp in p.into_inner() {
                            if matches!(tp.as_rule(), Rule::type_expr) {
                                params.push(parse_type_expr(tp.into_inner(), filename)?);
                            }
                        }
                    }
                    Rule::type_expr => {
                        return_type = Box::new(parse_type_expr(p.into_inner(), filename)?);
                    }
                    _ => {}
                }
            }

            Ok(TypeExpr::Fun(params, return_type))
        }
        Rule::array_type => {
            let elem_type = parse_type_expr_from_pair(pair.into_inner().next().unwrap(), filename)?;
            Ok(TypeExpr::Array(Box::new(elem_type)))
        }
        Rule::named_type => {
            parse_named_type(pair, filename)
        }
        _ => parse_type_expr_from_pair(pair, filename),
    }
}

fn parse_type_expr_from_pair(pair: Pair<Rule>, filename: &str) -> Result<TypeExpr, YolangError> {
    let span = span_of(&pair, filename);

    match pair.as_rule() {
        Rule::named_type => parse_named_type(pair, filename),
        Rule::ident => {
            let name = pair.as_str().to_string();
            Ok(TypeExpr::Named(name, vec![]))
        }
        _ => Err(YolangError::parse("Invalid type expression", &span)),
    }
}

fn parse_named_type(pair: Pair<Rule>, filename: &str) -> Result<TypeExpr, YolangError> {
    let span = span_of(&pair, filename);
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or_else(|| YolangError::parse("Missing type name", &span))?.as_str().to_string();

    let mut type_args = vec![];
    for p in inner {
        if matches!(p.as_rule(), Rule::type_args) {
            for tp in p.into_inner() {
                if matches!(tp.as_rule(), Rule::type_expr) {
                    type_args.push(parse_type_expr(tp.into_inner(), filename)?);
                }
            }
        }
    }

    Ok(TypeExpr::Named(name, type_args))
}

fn parse_type_path(pairs: Pairs<Rule>) -> Vec<String> {
    let mut parts = vec![];
    for pair in pairs {
        if matches!(pair.as_rule(), Rule::ident) {
            parts.push(pair.as_str().to_string());
        }
    }
    parts
}

// ── Operator parsing ────────────────────────────────────────────────────────

fn parse_binary_op(pair: &Pair<Rule>) -> BinOp {
    match pair.as_str() {
        "+" => BinOp::Add,
        "-" => BinOp::Sub,
        "*" => BinOp::Mul,
        "/" => BinOp::Div,
        "%" => BinOp::Rem,
        "==" => BinOp::Eq,
        "!=" => BinOp::Ne,
        "<" => BinOp::Lt,
        "<=" => BinOp::Le,
        ">" => BinOp::Gt,
        ">=" => BinOp::Ge,
        "&&" | "and" => BinOp::And,
        "||" | "or" => BinOp::Or,
        ".." => BinOp::Range,
        "..=" => BinOp::RangeInclusive,
        _ => BinOp::Add, // fallback
    }
}

fn parse_assign_op(s: &str) -> AssignOp {
    match s {
        "=" => AssignOp::Assign,
        "+=" => AssignOp::AddAssign,
        "-=" => AssignOp::SubAssign,
        "*=" => AssignOp::MulAssign,
        "/=" => AssignOp::DivAssign,
        "%=" => AssignOp::RemAssign,
        _ => AssignOp::Assign,
    }
}

fn parse_unary_op(s: &str) -> UnaryOp {
    match s {
        "-" => UnaryOp::Neg,
        "!" => UnaryOp::Not,
        _ => UnaryOp::Not,
    }
}

// ── Helper functions ──────────────────────────────────────────────────────

fn expr_to_assign_target(expr: &Expr, _filename: &str) -> Result<AssignTarget, YolangError> {
    match expr {
        Expr::Ident(name, span) => Ok(AssignTarget::Ident(name.clone(), span.clone())),
        Expr::FieldAccess { object, field, span } => {
            Ok(AssignTarget::FieldAccess { object: object.clone(), field: field.clone(), span: span.clone() })
        }
        Expr::Index { object, index, span } => {
            Ok(AssignTarget::Index { object: object.clone(), index: index.clone(), span: span.clone() })
        }
        _ => Err(YolangError::parse("Invalid assignment target", expr.span())),
    }
}

fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                match next {
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    'r' => {
                        result.push('\r');
                        chars.next();
                    }
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    '"' => {
                        result.push('"');
                        chars.next();
                    }
                    _ => result.push(c),
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
