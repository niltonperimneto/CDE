use crate::ast::*;
use anyhow::Result;
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

// --- Parsers ---

fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

fn quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(take_until("\""), |s: &str| s.to_string()),
        char('"'),
    )(input)
}

fn ptype_header(input: &str) -> IResult<&str, String> {
    preceded(ws(tag("ptype")), ws(identifier))(input)
}

fn start_string(input: &str) -> IResult<&str, String> {
    preceded(ws(tag("start")), terminated(quoted_string, ws(char(';'))))(input)
}

fn action_enum(input: &str) -> IResult<&str, Action> {
    alt((
        value(Action::Start, tag("start")),
        value(Action::Queue, tag("queue")),
        value(Action::Observe, tag("observe")),
    ))(input)
}

fn scope_enum(input: &str) -> IResult<&str, Scope> {
    alt((
        value(Scope::Session, tag("session")),
        value(Scope::File, tag("file")),
        value(Scope::Both, tag("all")), // 'all' usually maps to both
    ))(input)
}

fn arg_mode(input: &str) -> IResult<&str, ArgMode> {
    alt((
        value(ArgMode::InOut, tag("inout")),
        value(ArgMode::In, tag("in")),
        value(ArgMode::Out, tag("out")),
    ))(input)
}

fn argument(input: &str) -> IResult<&str, Arg> {
    map(
        tuple((
            ws(arg_mode),
            ws(identifier),      // vtype
            opt(ws(identifier)), // name (optional)
        )),
        |(mode, vtype, name)| Arg { mode, vtype, name },
    )(input)
}

fn signature(input: &str) -> IResult<&str, Signature> {
    let (input, scope) = ws(scope_enum)(input)?;
    let (input, op) = ws(identifier)(input)?;
    let (input, args) = delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), argument),
        ws(char(')')),
    )(input)?;

    let (input, _) = ws(tag("=>"))(input)?;
    let (input, action) = ws(action_enum)(input)?;

    // opnum parser: opnum = 123
    let (input, _) = ws(tag("opnum"))(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, opnum) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    let (input, _) = ws(char(';'))(input)?;

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
    let (input, _) = ws(char('{'))(input)?;

    // Optional properties like 'start'
    let (input, start_str) = opt(start_string)(input)?;

    // 'handle:' block
    let (input, _) = opt(tuple((ws(tag("handle")), ws(char(':')))))(input)?;

    let (input, signatures) = many0(signature)(input)?;

    let (input, _) = ws(char('}'))(input)?;

    // Eat trailing semicolon if any
    let (input, _) = opt(ws(char(';')))(input)?;

    Ok((
        input,
        Ptype {
            name,
            start_string: start_str,
            signatures,
        },
    ))
}

// Strip C-style comments /* ... */
fn strip_comments(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
                          // scan until '*/'
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

pub fn parse_ptype(input: &str) -> Result<Ptype> {
    let cleaned = strip_comments(input);
    let (_, ptype) = ptype_block(&cleaned).map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    Ok(ptype)
}
