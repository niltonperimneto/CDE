//! Diagnostic errors for BIL parsing.

use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("invalid BIL file: {message}")]
#[diagnostic(
    code(cde::dtappbuilder::bil::parse_error),
    help("BIL files are `:keyword value` lists produced by dtappbuilder — see `programs/dtappbuilder/src/libABil/bil_parse.y`")
)]
pub struct BilError {
    #[source_code]
    pub src: NamedSource,
    #[label("here")]
    pub span: SourceSpan,
    pub message: String,
}

impl BilError {
    pub fn new(input: &str, offset: usize, message: impl Into<String>) -> Self {
        let len = input.len().saturating_sub(offset).max(1);
        Self {
            src: NamedSource::new("<bil file>", input.to_string()),
            span: SourceSpan::new(offset.into(), len.into()),
            message: message.into(),
        }
    }
}
