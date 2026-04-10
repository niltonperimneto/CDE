//! Tokeniser for CSA recurrence rules.
//!
//! Replaces the hand-written scanner in `lib/csa/rescan.c`. Produces a flat
//! stream of [`Token`]s along with byte offsets for diagnostic reporting.

use crate::ast::Token;
use crate::error::{LexError, LexErrorKind};

#[derive(Debug, Clone)]
pub struct Lexed {
    pub token: Token,
    pub offset: usize,
    pub len: usize,
}

pub fn tokenize(input: &str) -> Result<Vec<Lexed>, LexError> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i];

        // Skip whitespace.
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        let start = i;

        // Single-character punctuation.
        if c == b'$' {
            out.push(Lexed { token: Token::EndMarker, offset: start, len: 1 });
            i += 1;
            continue;
        }
        if c == b'#' {
            out.push(Lexed { token: Token::Duration, offset: start, len: 1 });
            i += 1;
            continue;
        }

        // Numbers (may be followed by + or - to form a week occurrence).
        if c.is_ascii_digit() {
            let mut j = i;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            // Followed by `+` or `-` → week occurrence marker.
            if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
                let digit = std::str::from_utf8(&bytes[i..j])
                    .map_err(|_| LexError::at(start, LexErrorKind::InvalidUtf8))?;
                let num: u32 = digit
                    .parse()
                    .map_err(|_| LexError::at(start, LexErrorKind::BadNumber))?;
                let front = bytes[j] == b'+';
                let tok = occurrence_token(num, front)
                    .ok_or_else(|| LexError::at(start, LexErrorKind::BadOccurrence))?;
                out.push(Lexed { token: tok, offset: start, len: (j - i) + 1 });
                i = j + 1;
                continue;
            }

            // Otherwise this is either a plain NUMBER or the leading digits
            // of an ISO-8601 date (contains '-' or 'T' later).
            let num_str = std::str::from_utf8(&bytes[i..j])
                .map_err(|_| LexError::at(start, LexErrorKind::InvalidUtf8))?;
            // Peek past the number — if we hit a non-space/non-delimiter we
            // are inside an ISO date literal (e.g. 19960101T000000Z).
            if j < bytes.len()
                && (bytes[j].is_ascii_alphabetic() || bytes[j] == b'-')
            {
                // Read until delimiter.
                let mut k = j;
                while k < bytes.len() && !bytes[k].is_ascii_whitespace() && bytes[k] != b'$' {
                    k += 1;
                }
                let date = std::str::from_utf8(&bytes[i..k])
                    .map_err(|_| LexError::at(start, LexErrorKind::InvalidUtf8))?;
                out.push(Lexed {
                    token: Token::Date(date.to_string()),
                    offset: start,
                    len: k - i,
                });
                i = k;
                continue;
            }

            let num: u32 = num_str
                .parse()
                .map_err(|_| LexError::at(start, LexErrorKind::BadNumber))?;
            out.push(Lexed { token: Token::Number(num), offset: start, len: j - i });
            i = j;
            continue;
        }

        // Alphabetic → command or weekday.
        if c.is_ascii_alphabetic() {
            let mut j = i;
            while j < bytes.len() && bytes[j].is_ascii_alphabetic() {
                j += 1;
            }
            let word = std::str::from_utf8(&bytes[i..j])
                .map_err(|_| LexError::at(start, LexErrorKind::InvalidUtf8))?;
            let tok = keyword_token(word)
                .ok_or_else(|| LexError::at(start, LexErrorKind::UnknownKeyword))?;
            out.push(Lexed { token: tok, offset: start, len: j - i });
            i = j;
            continue;
        }

        return Err(LexError::at(start, LexErrorKind::UnexpectedChar(c as char)));
    }

    Ok(out)
}

fn keyword_token(word: &str) -> Option<Token> {
    // Multi-letter commands first (MP, MD, YD, YM, LD, SU, MO, ...).
    match word {
        "M"  => Some(Token::MinuteCommand),
        "D"  => Some(Token::DailyCommand),
        "W"  => Some(Token::WeeklyCommand),
        "MP" => Some(Token::MonthPosCommand),
        "MD" => Some(Token::MonthDayCommand),
        "YD" => Some(Token::YearDayCommand),
        "YM" => Some(Token::YearMonthCommand),
        "LD" => Some(Token::LastDay),
        "SU" => Some(Token::Sunday),
        "MO" => Some(Token::Monday),
        "TU" => Some(Token::Tuesday),
        "WE" => Some(Token::Wednesday),
        "TH" => Some(Token::Thursday),
        "FR" => Some(Token::Friday),
        "SA" => Some(Token::Saturday),
        _ => None,
    }
}

fn occurrence_token(num: u32, front: bool) -> Option<Token> {
    Some(match (num, front) {
        (1, true) => Token::FirstWeek,
        (2, true) => Token::SecondWeek,
        (3, true) => Token::ThirdWeek,
        (4, true) => Token::FourthWeek,
        (5, true) => Token::FifthWeek,
        (1, false) => Token::LastWeek,
        (2, false) => Token::SecondLast,
        (3, false) => Token::ThirdLast,
        (4, false) => Token::FourthLast,
        (5, false) => Token::FifthLast,
        _ => return None,
    })
}
