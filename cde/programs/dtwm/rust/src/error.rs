//! miette-powered diagnostic errors for dtwmrc parsing.

use miette::{Diagnostic, NamedSource, SourceSpan};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("failed to parse dtwmrc resource file")]
#[diagnostic(
    code(cde::dtwm::parse_error),
    help("check for matching braces on every Menu / Keys / Buttons block")
)]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource,

    #[label("here")]
    pub span: SourceSpan,

    pub message: String,
    pub file: PathBuf,
}

impl ParseError {
    pub fn from_pest(
        file: PathBuf,
        source: String,
        err: pest::error::Error<crate::parser::Rule>,
    ) -> Self {
        let offset = match err.location {
            pest::error::InputLocation::Pos(p) => p,
            pest::error::InputLocation::Span((s, _)) => s,
        };
        let len = match err.location {
            pest::error::InputLocation::Pos(_) => 1,
            pest::error::InputLocation::Span((s, e)) => e.saturating_sub(s).max(1),
        };
        let name = file.display().to_string();
        ParseError {
            src: NamedSource::new(name, source),
            span: SourceSpan::new(offset.into(), len.into()),
            message: err.variant.message().into_owned(),
            file,
        }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum DtwmError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parse(#[from] ParseError),

    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
