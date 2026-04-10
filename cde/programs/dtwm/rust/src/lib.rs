//! Memory-safe replacement for the CDE dtwm resource parser (dtwmrc).
//!
//! The legacy implementation in `programs/dtwm/WmParse.c`, `WmResParse.c`
//! and `Parse.c` manually tokenises dtwmrc files character-by-character
//! with fixed-size line buffers and hard-coded nesting limits. This crate
//! replaces it with:
//!
//! * A PEG grammar in `grammar/dtwmrc.pest` that identifies top-level
//!   declarations (`Menu`, `Keys`, `Buttons`, `PANEL`, `BOX`, `CONTROL`,
//!   `SWITCH`) and captures each body as raw text.
//! * A streaming second-pass tokeniser in `body.rs` that produces
//!   `BodyItem` values for each logical line, handling quoting, comments,
//!   and backslash line-continuation the same way as the legacy C tokeniser
//!   but with zero unsafe code.
//! * A C-ABI entry surface in `ffi.rs` that returns opaque handles so the
//!   C callers never dereference Rust structs directly.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod ast;
pub mod body;
pub mod error;
pub mod ffi;
pub mod parser;

pub use ast::{BodyItem, DeclKind, Declaration, ResourceFile};
pub use error::{DtwmError, ParseError};
pub use parser::{parse_file, parse_source};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SAMPLE: &str = r#"
# Sample dtwmrc
Menu DtRootMenu
{
    "Shuffle Up"    f.circle_up
    "Refresh"       f.refresh
    no-label        f.separator
    "Exit..."       f.exit
}

Keys DtKeyBindings
{
    Alt<Key>F4      window      f.kill
    Alt<Key>space   icon|window f.post_wmenu
}

Buttons DtButtonBindings
{
    <Btn3Down>      root        f.menu  DtRootMenu
}
"#;

    #[test]
    fn parses_sample_dtwmrc() {
        let rf = parse_source(SAMPLE, Path::new("sys.dtwmrc")).expect("parse ok");
        assert_eq!(rf.declarations.len(), 3);
        assert_eq!(rf.declarations[0].kind, DeclKind::Menu);
        assert_eq!(rf.declarations[0].name, "DtRootMenu");
        assert_eq!(rf.declarations[1].kind, DeclKind::Keys);
        assert_eq!(rf.declarations[2].kind, DeclKind::Buttons);
    }

    #[test]
    fn root_menu_items_tokenised() {
        let rf = parse_source(SAMPLE, Path::new("sys.dtwmrc")).unwrap();
        let menu = rf.find(DeclKind::Menu, "DtRootMenu").expect("found menu");
        let items = menu.items();
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].tokens, vec!["Shuffle Up", "f.circle_up"]);
        assert_eq!(items[3].tokens, vec!["Exit...", "f.exit"]);
    }

    #[test]
    fn panel_front_panel_grammar() {
        let src = r#"
PANEL FrontPanel {
  BOX Top {
    CONTROL Clock { TYPE clock }
  }
}
"#;
        let rf = parse_source(src, Path::new("fp.dtwmrc")).unwrap();
        // Top-level only captures PANEL; BOX/CONTROL live inside the body.
        assert_eq!(rf.declarations.len(), 1);
        assert_eq!(rf.declarations[0].kind, DeclKind::Panel);
        assert_eq!(rf.declarations[0].name, "FrontPanel");
        assert!(rf.declarations[0].body.contains("BOX"));
        assert!(rf.declarations[0].body.contains("CONTROL"));
    }

    #[test]
    fn missing_brace_returns_diagnostic() {
        let bad = "Menu Broken {\n  \"foo\" f.exec \"bar\"\n"; // no closing brace
        let err = parse_source(bad, Path::new("bad.dtwmrc")).unwrap_err();
        assert!(!err.message.is_empty());
    }
}
