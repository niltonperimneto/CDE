//! AST mirroring `rerule.h` from the legacy CSA (Calendar) library.
//!
//! The original `RepeatEvent` in C is a heterogeneous struct whose semantics
//! depend on a `RepeatType` tag — effectively a hand-rolled discriminated
//! union. Here we model it as a proper Rust `enum` so every variant carries
//! exactly the data it needs and invalid combinations are unrepresentable.

/// Number encoded with the "end-marker" flag bit from the legacy format.
///
/// The legacy parser sets the high bit of certain numbers to mean "this is
/// the *last* occurrence in the series" (`RE_SET_FLAG`). We model that bit
/// explicitly so consumers can see both pieces of information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlaggedNumber {
    pub value: u32,
    pub end_marker: bool,
}

impl FlaggedNumber {
    pub fn new(value: u32) -> Self { Self { value, end_marker: false } }
    pub fn flagged(value: u32) -> Self { Self { value, end_marker: true } }
}

/// Day of the week (matches `WeekDay` in `rerule.h`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekDay {
    Sun, Mon, Tue, Wed, Thu, Fri, Sat,
}

impl WeekDay {
    pub fn from_token(tok: Token) -> Option<Self> {
        match tok {
            Token::Sunday => Some(Self::Sun),
            Token::Monday => Some(Self::Mon),
            Token::Tuesday => Some(Self::Tue),
            Token::Wednesday => Some(Self::Wed),
            Token::Thursday => Some(Self::Thu),
            Token::Friday => Some(Self::Fri),
            Token::Saturday => Some(Self::Sat),
            _ => None,
        }
    }
}

/// Ordinal position of a week inside a month.
///
/// Maps 1:1 onto the `WK_F1..WK_F5` (first five) and `WK_L1..WK_L5`
/// (last five) constants in `rerule.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekOccurrence {
    First, Second, Third, Fourth, Fifth,
    Last, SecondLast, ThirdLast, FourthLast, FifthLast,
}

/// A (weekday, time) pair for weekly events. `times` may be empty, meaning
/// "no specific time — inherit from the appointment".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekdayTime {
    pub day: WeekDay,
    pub times: Vec<u32>,
}

/// A grouping of week occurrences + weekdays + times for monthly-by-position
/// events (e.g. "1+ MO 1300" = first Monday at 13:00).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekDayTimeList {
    pub occurrences: Vec<WeekOccurrence>,
    pub weekdays: Vec<WeekDay>,
    pub times: Vec<u32>,
}

/// Duration: either a fixed number of occurrences, the sentinel "infinity",
/// or "not set" (the legacy default when both duration and end-date are
/// absent, triggering a default of 2 in the old code).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    NotSet,
    Infinity,
    Count(u32),
}

/// A parsed recurrence rule.
///
/// This is a sum type of every variant produced by the legacy grammar in
/// `reparser.y`. The corresponding `RepeatType` constants from `rerule.h`
/// are annotated on each variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeatEvent {
    /// `M<interval> #<count>` — every N minutes.
    Minute {
        interval: u32,
        duration: Duration,
    },
    /// `D<interval> [times...] #<count>` — every N days, optionally at
    /// specific times of day.
    Daily {
        interval: u32,
        times: Vec<FlaggedNumber>,
        duration: Duration,
    },
    /// `W<interval> [weekday-times...] #<count>` — every N weeks.
    Weekly {
        interval: u32,
        weekday_times: Vec<WeekdayTime>,
        duration: Duration,
    },
    /// `MP<interval> [weekday-time-list...] #<count>` — monthly by position.
    MonthlyByPos {
        interval: u32,
        week_day_time: Vec<WeekDayTimeList>,
        duration: Duration,
    },
    /// `MD<interval> [day-of-month-list...] #<count>` — monthly by day.
    MonthlyByDay {
        interval: u32,
        days_of_month: Vec<FlaggedNumber>,
        duration: Duration,
    },
    /// `YM<interval> [month-list...] #<count>` — yearly by month.
    YearlyByMonth {
        interval: u32,
        months: Vec<FlaggedNumber>,
        duration: Duration,
    },
    /// `YD<interval> [day-of-year-list...] #<count>` — yearly by day.
    YearlyByDay {
        interval: u32,
        days_of_year: Vec<FlaggedNumber>,
        duration: Duration,
    },
}

impl RepeatEvent {
    pub fn with_duration(mut self, d: Duration) -> Self {
        match &mut self {
            RepeatEvent::Minute { duration, .. }
            | RepeatEvent::Daily { duration, .. }
            | RepeatEvent::Weekly { duration, .. }
            | RepeatEvent::MonthlyByPos { duration, .. }
            | RepeatEvent::MonthlyByDay { duration, .. }
            | RepeatEvent::YearlyByMonth { duration, .. }
            | RepeatEvent::YearlyByDay { duration, .. } => {
                *duration = d;
            }
        }
        self
    }
}

/// The top-level result of parsing a full recurrence rule — an event plus
/// an optional ISO-8601 end date (as a raw string; date parsing itself is
/// left to the caller so we can keep the recurrence crate dependency-free).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub event: RepeatEvent,
    pub end_date: Option<String>,
}

/// Tokens produced by [`crate::lexer`]. Mirrors the Yacc tokens in
/// `reparser.y:46-51` plus the numeric/date literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    MinuteCommand,
    DailyCommand,
    WeeklyCommand,
    MonthPosCommand,
    MonthDayCommand,
    YearDayCommand,
    YearMonthCommand,
    LastDay,
    FirstWeek, SecondWeek, ThirdWeek, FourthWeek, FifthWeek,
    LastWeek, SecondLast, ThirdLast, FourthLast, FifthLast,
    Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday,
    Number(u32),
    Date(String),
    Duration,   // '#'
    EndMarker,  // '$'
}
