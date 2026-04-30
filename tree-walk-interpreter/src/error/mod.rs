use thiserror::Error;

use crate::ast::Span;



/// All errors that can be produced at any stage of the pipeline.
#[derive(Debug, Error)]
pub enum YolangError {
    #[error("Parse error in {filename} at {start}..{end}: {message}")]
    ParseError { message: String, start: usize, end: usize,  filename: String },

    #[error("Parse error in {filename} at {start}..{end}, line {line}: {message}")]
    ParseErrorWithLine { message: String, start: usize, end: usize, line: String, filename: String },



    #[error("Type error in {filename} at {start}..{end}: {message}")]
    TypeError { message: String, start: usize, end: usize, filename: String },

    #[error("Panic at {filename} {start}..{end}: {message}")]
    RuntimePanic { message: String, start: usize, end: usize, filename: String },
    
    #[error("No match found. Todo: refactor out this error type.")]
    NoMatch,
}

impl YolangError {
    pub fn parse(msg: impl Into<String>, span: &Span) -> Self {
        Self::ParseError {
            message: msg.into(),
            start: span.start,
            end: span.end,
            filename: span.filename.clone(),
        }
    }

    pub fn type_error(msg: impl Into<String>, span: &Span) -> Self {
        Self::TypeError {
            message: msg.into(),
            start: span.start,
            end: span.end,
            filename: span.filename.clone(),
        }
    }

    pub fn panic(msg: impl Into<String>, span: &Span) -> Self {
        Self::RuntimePanic {
            message: msg.into(),
            start: span.start,
            end: span.end,
            filename: span.filename.clone(),
        }
    }
}
