//! AST for parsed `dtwmrc` declarations.
//!
//! The legacy dtwm parser (WmParse.c + WmResParse.c) produces two largely
//! independent views of a resource file: the top-level *declarations* (Menu,
//! Keys, Buttons, PANEL, ...) and the *body items* inside each declaration.
//! This module keeps the top-level structure typed and leaves the body as
//! raw text, matching the two-pass design of the original C loader.

use std::path::PathBuf;

/// Kind of top-level dtwmrc declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclKind {
    Menu,
    Keys,
    Buttons,
    Panel,
    Box_,
    Control,
    Switch,
}

impl DeclKind {
    pub fn as_str(self) -> &'static str {
        match self {
            DeclKind::Menu => "Menu",
            DeclKind::Keys => "Keys",
            DeclKind::Buttons => "Buttons",
            DeclKind::Panel => "PANEL",
            DeclKind::Box_ => "BOX",
            DeclKind::Control => "CONTROL",
            DeclKind::Switch => "SWITCH",
        }
    }
}

/// One top-level declaration. `body` is the raw text between the outer
/// braces — the second pass in `body.rs` interprets it on demand.
#[derive(Debug, Clone)]
pub struct Declaration {
    pub kind: DeclKind,
    pub name: String,
    pub body: String,
    pub source: PathBuf,
    pub line: u32,
}

impl Declaration {
    pub fn items(&self) -> Vec<BodyItem> {
        crate::body::parse_body(&self.body)
    }
}

/// One logical line inside a declaration body.
///
/// dtwmrc body lines take the form `tokens...` or `tokens... f.action args`.
/// This captures the raw token list; downstream code can match on the action
/// (e.g. `f.exec`, `f.menu`, `f.title`) to decide what to do with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyItem {
    pub tokens: Vec<String>,
}

/// Full parsed dtwmrc file.
#[derive(Debug, Default)]
pub struct ResourceFile {
    pub declarations: Vec<Declaration>,
}

impl ResourceFile {
    pub fn new() -> Self { Self::default() }

    pub fn find(&self, kind: DeclKind, name: &str) -> Option<&Declaration> {
        self.declarations.iter().find(|d| d.kind == kind && d.name == name)
    }

    pub fn menus(&self) -> impl Iterator<Item = &Declaration> {
        self.declarations.iter().filter(|d| d.kind == DeclKind::Menu)
    }
}
