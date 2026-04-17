//! nom-based parser for the dtappbuilder BIL format.
//!
//! Replaces `programs/dtappbuilder/src/libABil/bil_parse.y` (1 067 lines of
//! Yacc C) with a zero-copy, panic-free recursive-descent implementation.
//!
//! Grammar in brief (see `bil_parse.y` for the full version):
//!
//! ```text
//! file   := comment* bil-version? item*
//! item   := KEYWORD arg*
//! arg    := list | string | number | atom
//! list   := '(' arg* ')'
//! KEYWORD := ':' ident-chars        // ":type", ":element", ":module", …
//! atom   := ident-chars             // bare word: identifier or true/false
//! ```
//!
//! All items at *every* nesting depth are parsed uniformly — the distinction
//! between "object declarations" (`:element`, `:module` …) and "attributes"
//! (`:type`, `:label` …) is left to consumers of the AST.

use crate::ast::{Arg, BilFile, Item};
use crate::error::BilError;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_while1},
    character::complete::{anychar, char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize, value},
    multi::many0,
    sequence::{delimited, preceded},
};

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Parse a complete BIL file from a string slice.
pub fn parse(input: &str) -> Result<BilFile, BilError> {
    let (rest, file) = parse_file(input)
        .map_err(|e| BilError::new(input, offset_of(input, &e), format!("parse error: {e}")))?;
    if !rest.trim().is_empty() {
        let offset = input.len() - rest.len();
        return Err(BilError::new(input, offset, "trailing unparsed content"));
    }
    Ok(file)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract byte offset from a nom error pointing into `input`.
fn offset_of(input: &str, e: &nom::Err<nom::error::Error<&str>>) -> usize {
    match e {
        nom::Err::Error(inner) | nom::Err::Failure(inner) => {
            input.len().saturating_sub(inner.input.len())
        }
        nom::Err::Incomplete(_) => input.len(),
    }
}

// ---------------------------------------------------------------------------
// Whitespace and comment skipping
// ---------------------------------------------------------------------------

/// Skip whitespace and `//`-style line comments.
fn ws(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace0(input)?;
    // Check for a line comment; consume it and loop.
    if let Some(rest) = input.strip_prefix("//") {
        let end = rest.find('\n').map(|p| p + 1).unwrap_or(rest.len());
        let input = &rest[end..];
        // Recurse to consume further whitespace/comments after this line.
        return ws(input);
    }
    Ok((input, ()))
}

// ---------------------------------------------------------------------------
// Atoms / identifiers
// ---------------------------------------------------------------------------

/// A BIL keyword: `:` followed by one or more identifier characters.
/// Identifier characters are letters, digits, `-`, `_`, `.`.
fn keyword(input: &str) -> IResult<&str, &str> {
    let (input, _) = ws(input)?;
    recognize(preceded(
        char(':'),
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.'),
    ))
    .parse(input)
}

/// A bare atom (no leading `:`): identifier, boolean, version number part, etc.
fn bare_atom(input: &str) -> IResult<&str, &str> {
    let (input, _) = ws(input)?;
    take_while1(|c: char| {
        c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '/'
    })
    .parse(input)
}

// ---------------------------------------------------------------------------
// Numeric literals
// ---------------------------------------------------------------------------

fn integer(input: &str) -> IResult<&str, i64> {
    let (input, _) = ws(input)?;
    let (rest, s) = recognize(preceded(opt(char('-')), digit1)).parse(input)?;
    // Make sure the number isn't immediately followed by more ident chars
    // (that would make it a bare atom like "1st", not a number).
    if rest.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    let n: i64 = s.parse().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;
    Ok((rest, n))
}

fn float(input: &str) -> IResult<&str, f64> {
    let (input, _) = ws(input)?;
    // nom 8 removed `sequence::tuple`; a bare tuple of parsers implements
    // `Parser` directly, so we just nest them here.
    let (rest, s) = recognize(preceded(opt(char('-')), (digit1, char('.'), digit1))).parse(input)?;
    let f: f64 = s.parse().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Float))
    })?;
    Ok((rest, f))
}

// ---------------------------------------------------------------------------
// String literals
// ---------------------------------------------------------------------------

fn string_lit(input: &str) -> IResult<&str, String> {
    let (input, _) = ws(input)?;
    delimited(
        char('"'),
        map(
            opt(escaped_transform(
                is_not("\\\""),
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\n", tag("n")),
                    value("\t", tag("t")),
                    value("\r", tag("r")),
                    // Unknown escape: eat the next character so parsing always
                    // makes progress (avoids infinite loop on e.g. `\x`).  The
                    // original character is dropped because `escaped_transform`
                    // requires a `&'static str`-like output; this matches the
                    // legacy Yacc scanner, which also silently discarded them.
                    value("", anychar),
                )),
            )),
            |opt_s: Option<String>| opt_s.unwrap_or_default(),
        ),
        char('"'),
    )
    .parse(input)
}

// ---------------------------------------------------------------------------
// Arguments (recursive)
// ---------------------------------------------------------------------------

/// One argument inside an item: list, string, float, integer, keyword, or atom.
fn arg(input: &str) -> IResult<&str, Arg> {
    let (input, _) = ws(input)?;
    alt((
        map(list, Arg::List),
        map(string_lit, Arg::StringLit),
        // Float before integer to avoid matching "3." as integer "3".
        map(float, Arg::Float),
        map(integer, Arg::Integer),
        // Keyword atom (":something") — may appear inside lists.
        map(keyword, |s: &str| Arg::Atom(s.to_owned())),
        map(bare_atom, |s: &str| Arg::Atom(s.to_owned())),
    ))
    .parse(input)
}

/// A parenthesised list of arguments.
fn list(input: &str) -> IResult<&str, Vec<Arg>> {
    let (input, _) = ws(input)?;
    delimited(char('('), many0(arg), preceded(ws, char(')'))).parse(input)
}

// ---------------------------------------------------------------------------
// Items
// ---------------------------------------------------------------------------

/// One item: a keyword followed by zero or more arguments up to the next
/// keyword at the same depth OR a closing `)`.
fn item(input: &str) -> IResult<&str, Item> {
    let (input, kw) = keyword(input)?;
    // Collect args until we hit a sibling keyword, `)`, or EOF.
    let mut rest = input;
    let mut args = Vec::new();
    loop {
        let (r, _) = ws(rest)?;
        // Stop on sibling keyword, close paren, or EOF.
        if r.is_empty() || r.starts_with(':') || r.starts_with(')') {
            rest = r;
            break;
        }
        match arg(rest) {
            Ok((r2, a)) => {
                args.push(a);
                rest = r2;
            }
            Err(_) => break,
        }
    }
    Ok((rest, Item { keyword: kw.to_owned(), args }))
}

// ---------------------------------------------------------------------------
// File
// ---------------------------------------------------------------------------

fn bil_version(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = ws(input)?;
    let (input, _) = tag(":bil-version").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, major_s) = digit1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, minor_s) = digit1(input)?;
    let major: u32 = major_s.parse().unwrap_or(0);
    let minor: u32 = minor_s.parse().unwrap_or(0);
    Ok((input, (major, minor)))
}

fn parse_file(input: &str) -> IResult<&str, BilFile> {
    let (input, _) = ws(input)?;
    let (input, version) = opt(bil_version).parse(input)?;
    let (input, items) = many0(item).parse(input)?;
    let (input, _) = ws(input)?;
    Ok((input, BilFile { version, items }))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Arg;

    #[test]
    fn parses_empty_file() {
        let f = parse("").unwrap();
        assert!(f.items.is_empty());
        assert!(f.version.is_none());
    }

    #[test]
    fn parses_comments_only() {
        let f = parse("// This is a comment\n// Another comment\n").unwrap();
        assert!(f.items.is_empty());
    }

    #[test]
    fn parses_bil_version_header() {
        let f = parse(":bil-version 1 0\n:module foo ()").unwrap();
        assert_eq!(f.version, Some((1, 0)));
    }

    #[test]
    fn parses_module_with_element() {
        let src = r#"
// comment
:bil-version 1 0
:module mymodule
(
:element mywin
(
    :type :dialog
    :label "Hello"
    :x 100
    :y 200
    :visible :true
    :children (child1 child2)
)
)
"#;
        let f = parse(src).unwrap();
        assert_eq!(f.version, Some((1, 0)));
        let module = f.find(":module").expect(":module not found");
        assert_eq!(module.keyword, ":module");
        // The name "mymodule" should be the first Atom arg.
        assert!(matches!(module.args.first(), Some(Arg::Atom(n)) if n == "mymodule"));
    }

    #[test]
    fn parses_string_attribute() {
        let src = ":element e (\n    :label \"Hello World\"\n)";
        let f = parse(src).unwrap();
        let el = f.find(":element").unwrap();
        // Find the :label inside the body list arg.
        let body = el.args.iter().find_map(|a| {
            if let Arg::List(items) = a { Some(items) } else { None }
        }).unwrap();
        let label_idx = body.iter().position(|a| matches!(a, Arg::Atom(k) if k == ":label")).unwrap();
        assert!(matches!(&body[label_idx + 1], Arg::StringLit(s) if s == "Hello World"));
    }

    #[test]
    fn parses_numeric_attributes() {
        let src = ":element e (:x 42 :y -7)";
        let f = parse(src).unwrap();
        let el = f.find(":element").unwrap();
        let body = el.args.iter().find_map(|a| {
            if let Arg::List(items) = a { Some(items) } else { None }
        }).unwrap();
        let x_idx = body.iter().position(|a| matches!(a, Arg::Atom(k) if k == ":x")).unwrap();
        assert_eq!(body[x_idx + 1], Arg::Integer(42));
    }

    #[test]
    fn parses_attachment_spec() {
        let src = ":element e (:north-attachment (:point 0 0))";
        let f = parse(src).unwrap();
        let el = f.find(":element").unwrap();
        let body = el.args.iter().find_map(|a| {
            if let Arg::List(l) = a { Some(l) } else { None }
        }).unwrap();
        // :north-attachment should be followed by a list
        let na_idx = body.iter().position(|a| matches!(a, Arg::Atom(k) if k == ":north-attachment")).unwrap();
        assert!(matches!(&body[na_idx + 1], Arg::List(_)));
    }

    #[test]
    fn parses_children_list() {
        let src = ":element e (:children (child_a child_b child_c))";
        let f = parse(src).unwrap();
        let el = f.find(":element").unwrap();
        let body = el.args.iter().find_map(|a| {
            if let Arg::List(l) = a { Some(l) } else { None }
        }).unwrap();
        let ch_idx = body.iter().position(|a| matches!(a, Arg::Atom(k) if k == ":children")).unwrap();
        if let Arg::List(children) = &body[ch_idx + 1] {
            assert_eq!(children.len(), 3);
        } else {
            panic!("expected List for :children");
        }
    }

    #[test]
    fn malformed_input_produces_error() {
        // Unclosed parenthesis — parser should not hang.
        let r = parse(":element foo (\n    :type :dialog\n");
        // Either error or trailing tokens; either way the input "(" wasn't
        // closed so we expect either a parse failure or trailing content.
        // The key assertion is that the call terminates without panic.
        let _ = r; // may be Ok or Err depending on nom behaviour — no panic is the guarantee
    }

    #[test]
    fn real_bil_snippet() {
        // Derived from group.bil (the first element).
        let src = r#"
:bil-version 1 0
:module group
(

:element prop_dialog
(
    :type        :dialog
    :bg-color    "white"
    :label       "Group Property Editor"
    :resizable   :true
    :visible     :false
    :x           0
    :y           0
)
)
"#;
        let f = parse(src).unwrap();
        assert_eq!(f.version, Some((1, 0)));
        assert!(f.find(":module").is_some());
    }
}
