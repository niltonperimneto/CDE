//! Memory-safe boolean query parser for DtSearch — a Rust-native replacement
//! for `lib/DtSearch/boolyac.y` + `lib/DtSearch/boolpars.c`.
//!
//! The legacy `boolyac.y` is a small (138-line) Yacc grammar that accepts
//! DtSearch boolean expressions:
//!
//! ```text
//!   expr : WORD
//!        | expr '&' expr        (AND)
//!        | expr '|' expr        (OR)
//!        | '~' expr             (NOT)
//!        | '(' expr ')'
//!        | WORD '@' N WORD      (COLLOC: proximity search)
//! ```
//!
//! This module reuses [`crate::search::QueryParser`], which already
//! implements the recursive-descent grammar, and adds:
//!
//! * a [`miette`]-powered diagnostic type, so parse errors highlight the
//!   offending span in the original query string instead of just returning
//!   a bare `String`,
//! * a [`parse_query`] entry point returning a `Result<Query, QueryError>`.
//!
//! The legacy `String`-returning `QueryParser::parse` is preserved for the
//! existing unit tests.

use crate::search::{Query, QueryParser};
use miette::{Diagnostic, NamedSource, SourceSpan};

/// Diagnostic error produced when a DtSearch boolean query fails to parse.
///
/// Carries the full original query text so the miette renderer can point at
/// the failing span with a caret in terminal output.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("invalid DtSearch boolean query: {message}")]
#[diagnostic(
    code(cde::dtsearch::query::parse_error),
    help("DtSearch boolean queries use `&` (AND), `|` (OR), `~` (NOT), parentheses, and `word1 @n word2` for collocation")
)]
pub struct QueryError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub span: SourceSpan,
    pub message: String,
}

impl QueryError {
    fn new(input: &str, message: impl Into<String>) -> Self {
        // The legacy parser doesn't track byte offsets; highlight the whole
        // query so the user at least sees the text they submitted.
        let len = input.len().max(1);
        Self {
            src: NamedSource::new("<dtsearch query>", input.to_string()),
            span: SourceSpan::new(0.into(), len.into()),
            message: message.into(),
        }
    }
}

/// Parse a DtSearch boolean query, returning a [`miette`] diagnostic on
/// failure.
pub fn parse_query(input: &str) -> Result<Query, QueryError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(QueryError::new(input, "empty query"));
    }
    QueryParser::new(input)
        .parse()
        .map_err(|e| QueryError::new(input, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_term() {
        let q = parse_query("hello").unwrap();
        assert!(matches!(q, Query::Term(_)));
    }

    #[test]
    fn empty_query_is_diagnostic() {
        let err = parse_query("   ").unwrap_err();
        assert!(!err.message.is_empty());
    }

    #[test]
    fn parses_collocation() {
        let q = parse_query("apple @3 pie").unwrap();
        match q {
            Query::Collocated { left, distance, right } => {
                assert_eq!(left, "apple");
                assert_eq!(distance, 3);
                assert_eq!(right, "pie");
            }
            other => panic!("expected Collocated, got {other:?}"),
        }
    }

    #[test]
    fn parses_collocation_inside_and() {
        // "(apple @5 pie) & bakery" — collocation then AND
        let q = parse_query("(apple @5 pie) & bakery").unwrap();
        assert!(matches!(q, Query::And(_, _)));
    }

    #[test]
    fn dangling_collocation_errors() {
        let err = parse_query("apple @3").unwrap_err();
        assert!(err.message.to_lowercase().contains("collocation"));
    }

    #[test]
    fn unclosed_paren_is_diagnostic() {
        let err = parse_query("(foo & bar").unwrap_err();
        assert!(!err.message.is_empty());
    }
}
