//! Memory-safe replacement for the CDE `.dt` (Action/DataType) configuration
//! parser that has historically lived in `lib/DtSvc/DtUtil1/ActionDb.c`.
//!
//! Design goals:
//!
//! * Declarative grammar. The full syntax lives in `grammar/dtfile.pest` and
//!   doubles as living documentation.
//! * Parallel directory scan using `rayon` + `walkdir`, replacing the legacy
//!   sequential loader to collapse CDE's multi-second cold-start.
//! * User-friendly diagnostics via `miette` so a missing brace produces a
//!   colour-coded source span instead of `Syntax error on line 42`.
//! * Opaque handles across the FFI boundary so the C callers (`dtaction`,
//!   `dtfile`, etc.) can never touch Rust memory directly.
//!
//! The public C ABI is generated into `include/libdt_parser.h` by the
//! workspace `build.rs` at compile time.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod ast;
pub mod error;
pub mod ffi;
pub mod loader;
pub mod parser;

pub use ast::{Database, Record, RecordKind};
pub use error::{DtError, ParseError};
pub use loader::{load_dir, load_search_path};
pub use parser::{parse_file, parse_source};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SAMPLE: &str = r#"
# Minimal example .dt file
ACTION Edit
{
    LABEL           Edit
    TYPE            COMMAND
    WINDOW_TYPE     PERM_TERMINAL
    EXEC_STRING     /usr/bin/vi "%Arg_1%"
    DESCRIPTION     "Opens the file in vi."
}

DATA_ATTRIBUTES TEXTFILE
{
    DESCRIPTION     "Plain text files"
    ICON            Dttext
    IS_TEXT         True
}
"#;

    #[test]
    fn parses_action_and_datatype() {
        let path = Path::new("test.dt");
        let records = parse_source(SAMPLE, path).expect("parse ok");
        assert_eq!(records.len(), 2);
        let action = &records[0];
        assert_eq!(action.name, "Edit");
        assert_eq!(action.kind, RecordKind::Action);
        assert_eq!(action.label(), Some("Edit"));
        assert_eq!(action.type_(), Some("COMMAND"));
        assert!(action.exec_string().unwrap().contains("/usr/bin/vi"));
        assert_eq!(action.description(), Some("Opens the file in vi."));

        let dtype = &records[1];
        assert_eq!(dtype.name, "TEXTFILE");
        assert_eq!(dtype.kind, RecordKind::DataAttributes);
        assert_eq!(dtype.get("ICON"), Some("Dttext"));
    }

    #[test]
    fn collects_records_into_database() {
        let path = Path::new("test.dt");
        let records = parse_source(SAMPLE, path).unwrap();
        let mut db = Database::new();
        for r in records {
            db.insert(r);
        }
        assert_eq!(db.action_count(), 1);
        assert_eq!(db.datatype_count(), 1);
        assert!(db.actions.contains_key("Edit"));
        assert!(db.data_attributes.contains_key("TEXTFILE"));
    }

    #[test]
    fn malformed_input_returns_miette_error() {
        let bad = "ACTION { }"; // missing name
        let err = parse_source(bad, Path::new("bad.dt")).unwrap_err();
        assert!(!err.message.is_empty());
    }

    #[test]
    fn comments_are_ignored() {
        let src = r#"
# top comment
ACTION Foo  # trailing comment is ignored
{
    LABEL foo
}
"#;
        let r = parse_source(src, Path::new("c.dt")).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].name, "Foo");
    }
}
