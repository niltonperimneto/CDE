//! Memory-safe replacement for the dtappbuilder BIL parser
//! (`programs/dtappbuilder/src/libABil/bil_parse.y`).
//!
//! The legacy parser is a 1 067-line Yacc grammar that turns `.bil` files —
//! the dtappbuilder UI serialisation format — into callback-driven C structs.
//! This crate provides:
//!
//! * A typed AST ([`ast::BilFile`], [`ast::Item`], [`ast::Arg`]).
//! * A zero-copy `nom`-based recursive-descent parser ([`parser`]).
//! * `miette`-powered diagnostics so a malformed file shows a labelled
//!   source span instead of a bare `yyerror` message.
//! * An opaque-handle C API ([`ffi`]) compatible with the existing
//!   `bilP_load_*` call-sites without touching Rust internals.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod ast;
pub mod error;
pub mod ffi;
pub mod parser;

pub use ast::{Arg, BilFile, Item};
pub use error::BilError;
pub use parser::parse;

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Round-trip tests on real BIL snippets derived from
    // `programs/dtappbuilder/src/ab/group.bil`.
    // -----------------------------------------------------------------------

    #[test]
    fn parses_group_bil_snippet() {
        let src = include_str!("../tests/group_snippet.bil");
        let f = parse(src).unwrap();
        assert_eq!(f.version, Some((1, 0)));
        assert!(f.find(":module").is_some(), "expected :module item");
    }

    #[test]
    fn parses_element_with_attachment() {
        let src = r#"
:bil-version 1 0
:element objlist_panel
(
    :type :container
    :container-type :relative
    :x 0
    :y 0
    :visible :true
    :north-attachment (:point 0 0)
    :east-attachment  (:obj prop_dialog 0)
    :west-attachment  (:point 0 0)
    :children (objlist_label objlist)
)
"#;
        let f = parse(src).unwrap();
        let el = f.find(":element").unwrap();
        assert_eq!(el.keyword, ":element");
        // First arg should be the name identifier.
        assert!(matches!(el.args.first(), Some(Arg::Atom(n)) if n == "objlist_panel"));
    }

    #[test]
    fn ffi_round_trip() {
        use std::ffi::CString;
        let src = CString::new(":bil-version 1 0\n:module test ()").unwrap();
        let h = unsafe { ffi::cde_bil_parse(src.as_ptr()) };
        assert!(!h.is_null(), "expected non-null handle");
        assert_eq!(unsafe { ffi::cde_bil_version_major(h) }, 1);
        assert_eq!(unsafe { ffi::cde_bil_version_minor(h) }, 0);
        assert!(unsafe { ffi::cde_bil_item_count(h) } > 0);
        unsafe { ffi::cde_bil_free(h) };
    }

    #[test]
    fn ffi_error_path() {
        use std::ffi::{CStr, CString};
        // Deeply nested unclosed parentheses — should not panic.
        let src = CString::new("((((((((not valid bil").unwrap();
        let h = unsafe { ffi::cde_bil_parse(src.as_ptr()) };
        // Either NULL (if we detect the error) or a best-effort partial parse.
        if h.is_null() {
            let err = ffi::cde_bil_last_error();
            assert!(!err.is_null());
            let msg = unsafe { CStr::from_ptr(err) }.to_string_lossy();
            assert!(!msg.is_empty());
        } else {
            unsafe { ffi::cde_bil_free(h) };
        }
    }
}
