// This crate is pure-safe Rust — no unsafe blocks anywhere.
use std::fmt;

/// Errors that can occur during XDR encoding or decoding.
///
/// The variant set mirrors the error surface of `xdr-codec 0.2` so that
/// generated code using `cde_xdr as xdr_codec` compiles unchanged.
#[derive(Debug)]
pub struct XdrError {
    kind: XdrErrorKind,
}

#[derive(Debug)]
enum XdrErrorKind {
    /// Underlying I/O failure (short read, closed connection, …)
    Io(std::io::Error),
    /// Discriminant value does not correspond to any known enum variant.
    InvalidEnum,
    /// Discriminant matched a known arm but the case payload was invalid.
    InvalidCase,
    /// Byte data is not valid UTF-8 (for XDR `string` types).
    InvalidUtf8(std::string::FromUtf8Error),
    /// Variable-length field exceeds the protocol-declared maximum length.
    SizeLimit { got: usize, max: usize },
}

impl XdrError {
    /// Construct an "invalid enum discriminant" error.
    ///
    /// Called by xdrgen-generated union match arms when the discriminant does
    /// not correspond to any declared case.  Accepts an optional discriminant
    /// value for diagnostics — the generated code passes the unrecognised
    /// value, e.g. `Error::invalidenum(e)`.
    pub fn invalidenum<T>(_discriminant: T) -> Self {
        Self { kind: XdrErrorKind::InvalidEnum }
    }

    /// Construct an "invalid union case" error.
    ///
    /// Called by xdrgen-generated code when the case value within a known
    /// discriminant arm cannot be decoded.
    pub fn invalidcase<T>(_discriminant: T) -> Self {
        Self { kind: XdrErrorKind::InvalidCase }
    }

    /// Construct a size-limit error.
    pub fn size_limit(got: usize, max: usize) -> Self {
        Self { kind: XdrErrorKind::SizeLimit { got, max } }
    }

    /// Returns `true` if this error originated from an I/O failure.
    pub fn is_io(&self) -> bool {
        matches!(self.kind, XdrErrorKind::Io(_))
    }
}

impl fmt::Display for XdrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            XdrErrorKind::Io(e) => write!(f, "XDR I/O error: {}", e),
            XdrErrorKind::InvalidEnum => write!(f, "XDR invalid enum discriminant"),
            XdrErrorKind::InvalidCase => write!(f, "XDR invalid union case"),
            XdrErrorKind::InvalidUtf8(e) => write!(f, "XDR invalid UTF-8 in string: {}", e),
            XdrErrorKind::SizeLimit { got, max } => {
                write!(f, "XDR size limit exceeded: got {} bytes, max {}", got, max)
            }
        }
    }
}

impl std::error::Error for XdrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            XdrErrorKind::Io(e) => Some(e),
            XdrErrorKind::InvalidUtf8(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for XdrError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: XdrErrorKind::Io(e) }
    }
}

impl From<std::string::FromUtf8Error> for XdrError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self { kind: XdrErrorKind::InvalidUtf8(e) }
    }
}
