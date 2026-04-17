//! ToolTalk `.ptype` source parser.
//!
//! This module consumes ToolTalk process-type declaration files (the ptype
//! grammar defined in the legacy `tt_type_comp` yacc grammar) and produces an
//! [`Ptype`] AST.  It replaces the yacc/flex toolchain with a pure-Rust
//! parser-combinator implementation built on `nom`.
//!
//! # nom version
//!
//! Pinned to `nom = "8"`.  The 8.x release dropped `sequence::tuple`; sequence
//! parsing is now done by calling `.parse()` on a bare tuple of parsers, and
//! most combinators consume their input via `Parser::parse` rather than being
//! direct `fn(input)`s.  Keep this in mind when editing.
//!
//! # Grammar (informal)
//!
//! ```text
//! ptype_file  := ptype_block+
//! ptype_block := "ptype" IDENT "{" [ "start" STRING ";" ]
//!                             [ "handle" ":" ] signature* "}" [ ";" ]
//! signature   := scope IDENT "(" arg_list ")" "=>" action
//!                             "opnum" "=" INT ";"
//! scope       := "session" | "file" | "all"
//! action      := "start"   | "queue" | "observe"
//! arg         := mode IDENT [ IDENT ]
//! mode        := "in"      | "out"   | "inout"
//! ```

use crate::ast::*;
use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
};

// ---------------------------------------------------------------------------
// Whitespace helpers
// ---------------------------------------------------------------------------

/// Wrap `inner` so leading and trailing whitespace are consumed.
fn ws<'a, F, O>(inner: F) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

// ---------------------------------------------------------------------------
// Lexemes
// ---------------------------------------------------------------------------

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(take_until("\""), |s: &str| s.to_string()),
        char('"'),
    )
    .parse(input)
}

fn ptype_header(input: &str) -> IResult<&str, String> {
    preceded(ws(tag("ptype")), ws(identifier)).parse(input)
}

fn start_string(input: &str) -> IResult<&str, String> {
    preceded(ws(tag("start")), terminated(quoted_string, ws(char(';')))).parse(input)
}

// ---------------------------------------------------------------------------
// Enum-like tokens
// ---------------------------------------------------------------------------

fn action_enum(input: &str) -> IResult<&str, Action> {
    alt((
        value(Action::Start, tag("start")),
        value(Action::Queue, tag("queue")),
        value(Action::Observe, tag("observe")),
    ))
    .parse(input)
}

fn scope_enum(input: &str) -> IResult<&str, Scope> {
    alt((
        value(Scope::Session, tag("session")),
        value(Scope::File, tag("file")),
        value(Scope::Both, tag("all")),
    ))
    .parse(input)
}

fn arg_mode(input: &str) -> IResult<&str, ArgMode> {
    // Order matters: "inout" must be tried before "in" because nom `alt` does
    // first-match not longest-match.
    alt((
        value(ArgMode::InOut, tag("inout")),
        value(ArgMode::In, tag("in")),
        value(ArgMode::Out, tag("out")),
    ))
    .parse(input)
}

// ---------------------------------------------------------------------------
// Composite rules
// ---------------------------------------------------------------------------

fn argument(input: &str) -> IResult<&str, Arg> {
    map(
        (ws(arg_mode), ws(identifier), opt(ws(identifier))),
        |(mode, vtype, name)| Arg { mode, vtype, name },
    )
    .parse(input)
}

fn signature(input: &str) -> IResult<&str, Signature> {
    let (input, scope) = ws(scope_enum).parse(input)?;
    let (input, op) = ws(identifier).parse(input)?;
    let (input, args) = delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), argument),
        ws(char(')')),
    )
    .parse(input)?;

    let (input, _) = ws(tag("=>")).parse(input)?;
    let (input, action) = ws(action_enum).parse(input)?;

    // opnum = N ;
    let (input, _) = ws(tag("opnum")).parse(input)?;
    let (input, _) = ws(char('=')).parse(input)?;
    let (input, opnum) = map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)?;
    let (input, _) = ws(char(';')).parse(input)?;

    Ok((
        input,
        Signature {
            scope,
            op,
            args,
            action,
            opnum,
        },
    ))
}

fn ptype_block(input: &str) -> IResult<&str, Ptype> {
    let (input, name) = ptype_header(input)?;
    let (input, _) = ws(char('{')).parse(input)?;

    // Optional `start "…";` property.
    let (input, start_str) = opt(start_string).parse(input)?;

    // Optional `handle:` label (legacy syntax).
    let (input, _) = opt((ws(tag("handle")), ws(char(':')))).parse(input)?;

    let (input, signatures) = many0(signature).parse(input)?;

    let (input, _) = ws(char('}')).parse(input)?;
    let (input, _) = opt(ws(char(';'))).parse(input)?;

    Ok((
        input,
        Ptype {
            name,
            start_string: start_str,
            signatures,
        },
    ))
}

// ---------------------------------------------------------------------------
// Preprocessing + public entry point
// ---------------------------------------------------------------------------

/// Remove C-style `/* … */` comments before parsing.  Line comments are not
/// part of the ptype grammar, so they are not stripped here.
fn strip_comments(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
            while let Some(c2) = chars.next() {
                if c2 == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse a single ptype declaration block.  Returns [`anyhow::Error`] on
/// syntactic failure; callers that need precise span information should use
/// the internal [`ptype_block`] combinator directly.
pub fn parse_ptype(input: &str) -> Result<Ptype> {
    let cleaned = strip_comments(input);
    let (_, ptype) = ptype_block(&cleaned).map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    Ok(ptype)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_ptype() {
        let src = r#"
            ptype Demo {
                start "demo_server";
                session Hello(in string msg) => start opnum = 1;
            }
        "#;
        let p = parse_ptype(src).expect("should parse");
        assert_eq!(p.name, "Demo");
        assert_eq!(p.start_string.as_deref(), Some("demo_server"));
        assert_eq!(p.signatures.len(), 1);
        let s = &p.signatures[0];
        assert_eq!(s.op, "Hello");
        assert_eq!(s.opnum, 1);
    }

    #[test]
    fn strips_block_comments() {
        let src = r#"ptype X { /* hello */ } ;"#;
        let p = parse_ptype(src).expect("should parse");
        assert_eq!(p.name, "X");
        assert!(p.start_string.is_none());
        assert!(p.signatures.is_empty());
    }

    #[test]
    fn inout_matches_before_in() {
        let src = r#"
            ptype A {
                session Op(inout string x) => queue opnum = 7;
            }
        "#;
        let p = parse_ptype(src).expect("should parse");
        assert_eq!(p.signatures[0].args[0].mode, ArgMode::InOut);
    }
}
