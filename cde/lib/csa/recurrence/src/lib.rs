//! Memory-safe replacement for the CSA calendar recurrence rule parser
//! (`lib/csa/reparser.y` + `lib/csa/rescan.c`).
//!
//! The legacy parser is a Yacc grammar — 1 080 lines of C — that turns
//! recurrence strings like `D1 1000 2000 #5 19960101T000000Z` into a
//! `RepeatEvent` struct tree. This crate provides:
//!
//! * A typed AST ([`RepeatEvent`], [`Rule`]) that makes invalid states
//!   unrepresentable.
//! * A small hand-written lexer ([`lexer`]) and recursive-descent parser
//!   ([`parser`]) with bounded recursion.
//! * `miette`-powered diagnostics so a malformed rule shows a labelled
//!   source span instead of the legacy `yyerror` stderr dump.
//! * An opaque-handle C API ([`ffi`]) that the legacy C callers can use
//!   without touching Rust memory directly.
//!
//! This crate is intentionally self-contained: no XDR, no RPC, no libtirpc
//! dependency. It can be linked into any CDE component that needs to
//! understand recurrence rules.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod ast;
pub mod error;
pub mod ffi;
pub mod lexer;
pub mod parser;

pub use ast::{
    Duration, FlaggedNumber, RepeatEvent, Rule, WeekDay, WeekOccurrence, WeekDayTimeList,
    WeekdayTime,
};
pub use error::{LexError, LexErrorKind, ParseError};
pub use parser::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minute_rule() {
        let r = parse("M10 #5").unwrap();
        assert_eq!(
            r.event,
            RepeatEvent::Minute { interval: 10, duration: Duration::Count(5) }
        );
        assert_eq!(r.end_date, None);
    }

    #[test]
    fn parses_daily_rule_with_times() {
        let r = parse("D1 1000 1400 #10").unwrap();
        match r.event {
            RepeatEvent::Daily { interval, times, duration } => {
                assert_eq!(interval, 1);
                assert_eq!(times.len(), 2);
                assert_eq!(times[0].value, 1000);
                assert_eq!(times[1].value, 1400);
                assert_eq!(duration, Duration::Count(10));
            }
            _ => panic!("expected Daily"),
        }
    }

    #[test]
    fn parses_weekly_with_weekdays() {
        let r = parse("W1 MO TU WE #4").unwrap();
        match r.event {
            RepeatEvent::Weekly { interval, weekday_times, duration } => {
                assert_eq!(interval, 1);
                assert_eq!(weekday_times.len(), 3);
                assert_eq!(weekday_times[0].day, WeekDay::Mon);
                assert_eq!(weekday_times[1].day, WeekDay::Tue);
                assert_eq!(weekday_times[2].day, WeekDay::Wed);
                assert_eq!(duration, Duration::Count(4));
            }
            _ => panic!("expected Weekly"),
        }
    }

    #[test]
    fn parses_monthly_by_day() {
        let r = parse("MD1 15 #12").unwrap();
        match r.event {
            RepeatEvent::MonthlyByDay { interval, days_of_month, duration } => {
                assert_eq!(interval, 1);
                assert_eq!(days_of_month.len(), 1);
                assert_eq!(days_of_month[0].value, 15);
                assert_eq!(duration, Duration::Count(12));
            }
            _ => panic!("expected MonthlyByDay"),
        }
    }

    #[test]
    fn parses_monthly_by_position() {
        let r = parse("MP1 1+ MO #6").unwrap();
        match r.event {
            RepeatEvent::MonthlyByPos { interval, week_day_time, duration } => {
                assert_eq!(interval, 1);
                assert_eq!(week_day_time.len(), 1);
                assert_eq!(week_day_time[0].occurrences, vec![WeekOccurrence::First]);
                assert_eq!(week_day_time[0].weekdays, vec![WeekDay::Mon]);
                assert_eq!(duration, Duration::Count(6));
            }
            _ => panic!("expected MonthlyByPos"),
        }
    }

    #[test]
    fn parses_infinite_duration() {
        let r = parse("D1 #0").unwrap();
        match r.event {
            RepeatEvent::Daily { duration, .. } => {
                assert_eq!(duration, Duration::Infinity);
            }
            _ => panic!("expected Daily"),
        }
    }

    #[test]
    fn parses_rule_without_duration() {
        let r = parse("D1 1000").unwrap();
        match r.event {
            RepeatEvent::Daily { duration, .. } => {
                assert_eq!(duration, Duration::NotSet);
            }
            _ => panic!("expected Daily"),
        }
    }

    #[test]
    fn parses_last_day_sentinel() {
        let r = parse("MD1 LD #2").unwrap();
        match r.event {
            RepeatEvent::MonthlyByDay { days_of_month, .. } => {
                assert_eq!(days_of_month.len(), 1);
                assert_eq!(days_of_month[0].value, 0x7FFF_FFFE);
            }
            _ => panic!("expected MonthlyByDay"),
        }
    }

    #[test]
    fn empty_rule_produces_error() {
        let e = parse("").unwrap_err();
        assert!(!e.message.is_empty());
    }

    #[test]
    fn unknown_command_produces_error() {
        let e = parse("XX1").unwrap_err();
        assert!(!e.message.is_empty());
    }

    #[test]
    fn ffi_round_trip() {
        use std::ffi::CString;
        let c = CString::new("D1 1000 #3").unwrap();
        let h = unsafe { ffi::csa_recurrence_parse(c.as_ptr()) };
        assert!(!h.is_null());
        assert_eq!(unsafe { ffi::csa_recurrence_type(h) }, ffi::CsaRepeatType::Daily as u32);
        assert_eq!(unsafe { ffi::csa_recurrence_interval(h) }, 1);
        assert_eq!(unsafe { ffi::csa_recurrence_duration(h) }, 3);
        unsafe { ffi::csa_recurrence_free(h) };
    }

    #[test]
    fn ffi_error_path_sets_last_error() {
        use std::ffi::{CStr, CString};
        let c = CString::new("not a rule").unwrap();
        let h = unsafe { ffi::csa_recurrence_parse(c.as_ptr()) };
        assert!(h.is_null());
        let err = ffi::csa_recurrence_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_string_lossy().into_owned();
        assert!(!msg.is_empty(), "expected non-empty error message");
    }
}
