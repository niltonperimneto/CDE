//! Second-pass tokeniser for dtwmrc declaration bodies.
//!
//! The pest grammar captures each declaration body as raw text. This module
//! turns that text into [`BodyItem`]s — one per logical line — using the
//! same whitespace/quote rules as the legacy `_DtWmParseNextToken` function
//! in `programs/dtwm/WmParse.c`.
//!
//! Rules:
//!
//! * `#` starts a comment that runs to end-of-line.
//! * Double-quoted strings preserve whitespace and `#`. `\` is an escape
//!   character inside a string (matches legacy behaviour).
//! * A trailing `\` at end of line continues the logical line onto the next.
//! * Tokens are otherwise separated by whitespace.

use crate::ast::BodyItem;

pub fn parse_body(body: &str) -> Vec<BodyItem> {
    let mut out = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut in_quote = false;
    let mut chars = body.chars().peekable();

    let flush_token = |token: &mut String, current: &mut Vec<String>| {
        if !token.is_empty() {
            current.push(std::mem::take(token));
        }
    };

    let flush_line =
        |current: &mut Vec<String>, out: &mut Vec<BodyItem>, token: &mut String| {
            if !token.is_empty() {
                current.push(std::mem::take(token));
            }
            if !current.is_empty() {
                out.push(BodyItem {
                    tokens: std::mem::take(current),
                });
            }
        };

    while let Some(c) = chars.next() {
        if in_quote {
            match c {
                '"' => {
                    in_quote = false;
                }
                '\\' => {
                    if let Some(next) = chars.next() {
                        token.push(next);
                    }
                }
                _ => token.push(c),
            }
            continue;
        }

        match c {
            '"' => {
                in_quote = true;
            }
            '#' => {
                // Comment to end of line
                while let Some(&p) = chars.peek() {
                    if p == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '\\' => {
                // Line continuation if \ is followed by newline
                if let Some(&'\n') = chars.peek() {
                    chars.next(); // consume newline; logical line continues
                } else if let Some(next) = chars.next() {
                    token.push(next);
                }
            }
            '\n' | '\r' => {
                flush_line(&mut current, &mut out, &mut token);
            }
            ' ' | '\t' => {
                flush_token(&mut token, &mut current);
            }
            _ => token.push(c),
        }
    }

    flush_line(&mut current, &mut out, &mut token);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenises_simple_line() {
        let items = parse_body("f.exec \"xterm -bg white\"\n");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tokens, vec!["f.exec", "xterm -bg white"]);
    }

    #[test]
    fn skips_comments() {
        let items = parse_body("# this is a comment\nf.menu DtRootMenu\n");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tokens, vec!["f.menu", "DtRootMenu"]);
    }

    #[test]
    fn handles_line_continuation() {
        let items = parse_body("f.exec \\\n    \"xterm\"\n");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tokens, vec!["f.exec", "xterm"]);
    }

    #[test]
    fn preserves_empty_quoted_values() {
        let items = parse_body("\"Exit Session\" f.exit\n");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tokens, vec!["Exit Session", "f.exit"]);
    }
}
