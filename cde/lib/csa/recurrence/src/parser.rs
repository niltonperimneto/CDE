//! Hand-written recursive-descent parser for CSA recurrence rules.
//!
//! Replaces the Yacc grammar in `lib/csa/reparser.y`. No unsafe code, no
//! recursion on untrusted input (depth is bounded by the fixed set of
//! grammar productions), and diagnostic errors via `miette`.

use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::{tokenize, Lexed};

pub fn parse(input: &str) -> Result<Rule, ParseError> {
    let tokens = tokenize(input).map_err(|e| ParseError::from_lex(input, e))?;
    let mut parser = Parser {
        input,
        tokens,
        pos: 0,
    };
    parser.parse_rule()
}

struct Parser<'a> {
    input: &'a str,
    tokens: Vec<Lexed>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|t| &t.token)
    }

    fn peek_lex(&self) -> Option<&Lexed> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Lexed> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn err<T>(&self, msg: impl Into<String>) -> Result<T, ParseError> {
        let (offset, len) = self
            .peek_lex()
            .map(|t| (t.offset, t.len))
            .unwrap_or((self.input.len(), 1));
        Err(ParseError::new(self.input, offset, len, msg))
    }

    fn expect_number(&mut self) -> Result<u32, ParseError> {
        match self.bump() {
            Some(Lexed { token: Token::Number(n), .. }) => Ok(n),
            Some(l) => Err(ParseError::new(
                self.input, l.offset, l.len, format!("expected a number, got {:?}", l.token))),
            None => self.err("expected a number"),
        }
    }

    fn parse_rule(&mut self) -> Result<Rule, ParseError> {
        let event = self.parse_event()?;
        let end_date = self.parse_end_date()?;
        if !self.eof() {
            return self.err("trailing tokens after recurrence rule");
        }
        Ok(Rule { event, end_date })
    }

    fn parse_event(&mut self) -> Result<RepeatEvent, ParseError> {
        let Some(tok) = self.peek().cloned() else {
            return self.err("empty rule");
        };
        match tok {
            Token::MinuteCommand => self.parse_minute(),
            Token::DailyCommand => self.parse_daily(),
            Token::WeeklyCommand => self.parse_weekly(),
            Token::MonthPosCommand => self.parse_monthly_pos(),
            Token::MonthDayCommand => self.parse_monthly_day(),
            Token::YearMonthCommand => self.parse_yearly_by_month(),
            Token::YearDayCommand => self.parse_yearly_by_day(),
            _ => self.err(format!("expected recurrence command, got {tok:?}")),
        }
    }

    fn parse_minute(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // M
        let interval = self.expect_number()?;
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::Minute { interval, duration })
    }

    fn parse_daily(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // D
        let interval = self.expect_number()?;
        let times = self.parse_number_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::Daily { interval, times, duration })
    }

    fn parse_weekly(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // W
        let interval = self.expect_number()?;
        let weekday_times = self.parse_weekday_time_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::Weekly { interval, weekday_times, duration })
    }

    fn parse_monthly_pos(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // MP
        let interval = self.expect_number()?;
        let week_day_time = self.parse_wdt_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::MonthlyByPos { interval, week_day_time, duration })
    }

    fn parse_monthly_day(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // MD
        let interval = self.expect_number()?;
        let days_of_month = self.parse_number_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::MonthlyByDay { interval, days_of_month, duration })
    }

    fn parse_yearly_by_month(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // YM
        let interval = self.expect_number()?;
        let months = self.parse_number_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::YearlyByMonth { interval, months, duration })
    }

    fn parse_yearly_by_day(&mut self) -> Result<RepeatEvent, ParseError> {
        self.bump(); // YD
        let interval = self.expect_number()?;
        let days_of_year = self.parse_number_list();
        let duration = self.parse_duration()?;
        Ok(RepeatEvent::YearlyByDay { interval, days_of_year, duration })
    }

    fn parse_duration(&mut self) -> Result<Duration, ParseError> {
        match self.peek() {
            Some(Token::Duration) => {
                self.bump();
                let n = self.expect_number()?;
                Ok(if n == 0 { Duration::Infinity } else { Duration::Count(n) })
            }
            _ => Ok(Duration::NotSet),
        }
    }

    fn parse_end_date(&mut self) -> Result<Option<String>, ParseError> {
        if let Some(Token::Date(_)) = self.peek() {
            if let Some(Lexed { token: Token::Date(d), .. }) = self.bump() {
                return Ok(Some(d));
            }
        }
        Ok(None)
    }

    fn parse_number_list(&mut self) -> Vec<FlaggedNumber> {
        let mut out = Vec::new();
        loop {
            match self.peek().cloned() {
                Some(Token::Number(n)) => {
                    self.bump();
                    let flagged = matches!(self.peek(), Some(Token::EndMarker));
                    if flagged {
                        self.bump();
                    }
                    out.push(if flagged {
                        FlaggedNumber::flagged(n)
                    } else {
                        FlaggedNumber::new(n)
                    });
                }
                Some(Token::LastDay) => {
                    self.bump();
                    let flagged = matches!(self.peek(), Some(Token::EndMarker));
                    if flagged {
                        self.bump();
                    }
                    // Sentinel value matching RE_LASTDAY in the legacy code.
                    const RE_LASTDAY: u32 = 0x7FFF_FFFE;
                    out.push(if flagged {
                        FlaggedNumber::flagged(RE_LASTDAY)
                    } else {
                        FlaggedNumber::new(RE_LASTDAY)
                    });
                }
                _ => break,
            }
        }
        out
    }

    fn parse_weekday_time_list(&mut self) -> Vec<WeekdayTime> {
        let mut out = Vec::new();
        while let Some(tok) = self.peek() {
            if let Some(day) = WeekDay::from_token(tok.clone()) {
                self.bump();
                let nums = self.parse_number_list();
                let times = nums.into_iter().map(|n| n.value).collect();
                out.push(WeekdayTime { day, times });
            } else {
                break;
            }
        }
        out
    }

    fn parse_wdt_list(&mut self) -> Vec<WeekDayTimeList> {
        let mut out = Vec::new();
        loop {
            let occurrences = self.parse_occurrence_list();
            if occurrences.is_empty() {
                break;
            }
            let mut weekdays = Vec::new();
            while let Some(tok) = self.peek() {
                if let Some(d) = WeekDay::from_token(tok.clone()) {
                    self.bump();
                    weekdays.push(d);
                } else {
                    break;
                }
            }
            let times: Vec<u32> = self
                .parse_number_list()
                .into_iter()
                .map(|n| n.value)
                .collect();
            out.push(WeekDayTimeList { occurrences, weekdays, times });
        }
        out
    }

    fn parse_occurrence_list(&mut self) -> Vec<WeekOccurrence> {
        let mut out = Vec::new();
        while let Some(tok) = self.peek() {
            let occ = match tok {
                Token::FirstWeek => WeekOccurrence::First,
                Token::SecondWeek => WeekOccurrence::Second,
                Token::ThirdWeek => WeekOccurrence::Third,
                Token::FourthWeek => WeekOccurrence::Fourth,
                Token::FifthWeek => WeekOccurrence::Fifth,
                Token::LastWeek => WeekOccurrence::Last,
                Token::SecondLast => WeekOccurrence::SecondLast,
                Token::ThirdLast => WeekOccurrence::ThirdLast,
                Token::FourthLast => WeekOccurrence::FourthLast,
                Token::FifthLast => WeekOccurrence::FifthLast,
                _ => break,
            };
            self.bump();
            out.push(occ);
        }
        out
    }
}
