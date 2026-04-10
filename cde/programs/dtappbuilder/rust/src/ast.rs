//! AST for the dtappbuilder BIL (Builder Intermediate Language) format.
//!
//! The BIL format is an S-expression-like DSL used by `dtappbuilder` to
//! serialise UI definitions. The `bil_parse.y` grammar describes it as a
//! sequence of *objects* (`:module`, `:element`, `:project`, `:composite`,
//! `:connection`, `:connection-list`) each containing *attribute* statements
//! (`:keyword value`).
//!
//! This module models that structure uniformly: every `:keyword` at any
//! nesting depth is an [`Item`], and its arguments are [`Arg`] values.

/// A scalar argument — a bare atom, string literal, or number.
#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    /// A bare word: an identifier or a `:keyword`.
    Atom(String),
    /// A double-quoted string (backslash escapes preserved as-is).
    StringLit(String),
    /// An integer literal (positive or negative).
    Integer(i64),
    /// A floating-point literal.
    Float(f64),
    /// A parenthesised list — e.g. `(foo bar)` for `:children`, or
    /// `(:point 0 0)` for attachment specs.
    List(Vec<Arg>),
}

/// One `:keyword` item and everything that follows it up to the next
/// top-level `:keyword` at the same nesting level.
///
/// An item maps to an `att` production in `bil_parse.y`:
/// ```text
///   att : :keyword  token
///       | :keyword  value_list          // (:keyword ...)
///       | :keyword  attach_list         // ((:point n m) ...)
///       | :keyword  simple_obj_body     // (:i18n (...))
///       | object_type  name  body       // (:element foo (...))
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    /// The keyword token including the leading `:` (e.g. `":type"`).
    pub keyword: String,
    /// Zero or more argument atoms, strings, numbers, and lists that
    /// follow the keyword before the next sibling keyword.
    pub args: Vec<Arg>,
}

/// The top-level representation of a parsed `.bil` file.
#[derive(Debug, Clone, PartialEq)]
pub struct BilFile {
    /// `:bil-version major minor` header, if present.
    pub version: Option<(u32, u32)>,
    /// All top-level items (declarations and attributes) in order.
    pub items: Vec<Item>,
}

impl BilFile {
    /// Return the first item whose keyword matches `kw` (case-sensitive).
    pub fn find(&self, kw: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.keyword == kw)
    }
}
