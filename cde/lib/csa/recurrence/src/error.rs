//! Diagnostic errors for CSA recurrence rule parsing.

use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    InvalidUtf8,
    BadNumber,
    BadOccurrence,
    UnknownKeyword,
    UnexpectedChar(char),
}

impl std::fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexErrorKind::InvalidUtf8 => write!(f, "input is not valid UTF-8"),
            LexErrorKind::BadNumber => write!(f, "invalid number literal"),
            LexErrorKind::BadOccurrence => write!(f, "week occurrence must be 1..5"),
            LexErrorKind::UnknownKeyword => write!(f, "unknown keyword"),
            LexErrorKind::UnexpectedChar(c) => write!(f, "unexpected character '{c}'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub offset: usize,
    pub kind: LexErrorKind,
}

impl LexError {
    pub fn at(offset: usize, kind: LexErrorKind) -> Self {
        Self { offset, kind }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at offset {}", self.kind, self.offset)
    }
}

impl std::error::Error for LexError {}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("invalid CSA recurrence rule: {message}")]
#[diagnostic(
    code(cde::csa::recurrence::parse_error),
    help("recurrence rules follow the grammar documented in rerule.h — see `csa_recurrence::RepeatEvent`")
)]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource,
    #[label("here")]
    pub span: SourceSpan,
    pub message: String,
}

impl ParseError {
    pub fn new(input: &str, offset: usize, len: usize, message: impl Into<String>) -> Self {
        Self {
            src: NamedSource::new("<recurrence rule>", input.to_string()),
            span: SourceSpan::new(offset.into(), len.max(1).into()),
            message: message.into(),
        }
    }

    pub fn from_lex(input: &str, e: LexError) -> Self {
        Self::new(input, e.offset, 1, e.to_string())
    }
}
