//! C FFI for the CSA recurrence parser.
//!
//! Exposes the same semantic surface as the legacy `_DtCm_repeat_info`
//! global + yyparse() pair, but with an opaque handle and explicit
//! ownership. All Rust panics are caught at the boundary.

use crate::ast::{Duration, RepeatEvent, Rule};
use crate::parser::parse;
use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{self, AssertUnwindSafe};
use std::ptr;

/// Opaque handle returned to C.
pub struct CsaRecurrenceHandle {
    pub(crate) rule: Rule,
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: impl Into<String>) {
    let c = CString::new(msg.into()).unwrap_or_else(|_| CString::new("<invalid>").unwrap());
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(c));
}

fn clear_last_error() {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}

/// Return the most recent parse error for this thread, or NULL.
///
/// # Safety
/// Returned pointer is owned by the thread-local; do not free.
#[no_mangle]
pub extern "C" fn csa_recurrence_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(ptr::null())
    })
}

/// Parse a recurrence rule string. Returns NULL on failure; call
/// [`csa_recurrence_last_error`] to retrieve the diagnostic.
///
/// # Safety
/// `input` must be a NUL-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn csa_recurrence_parse(input: *const c_char) -> *mut CsaRecurrenceHandle {
    clear_last_error();
    if input.is_null() {
        set_last_error("csa_recurrence_parse: NULL input");
        return ptr::null_mut();
    }
    let res = panic::catch_unwind(AssertUnwindSafe(|| {
        let s = match unsafe { CStr::from_ptr(input) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("csa_recurrence_parse: input not UTF-8");
                return ptr::null_mut();
            }
        };
        match parse(s) {
            Ok(rule) => Box::into_raw(Box::new(CsaRecurrenceHandle { rule })),
            Err(e) => {
                set_last_error(format!("{e:?}"));
                ptr::null_mut()
            }
        }
    }));
    match res {
        Ok(p) => p,
        Err(_) => {
            set_last_error("csa_recurrence_parse: Rust panic");
            ptr::null_mut()
        }
    }
}

/// Free a handle previously returned by [`csa_recurrence_parse`].
///
/// # Safety
/// `handle` must be NULL or a pointer previously returned here.
#[no_mangle]
pub unsafe extern "C" fn csa_recurrence_free(handle: *mut CsaRecurrenceHandle) {
    if handle.is_null() {
        return;
    }
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        drop(unsafe { Box::from_raw(handle) });
    }));
}

/// Integer tag mirroring the legacy `RepeatType` constants in `rerule.h`.
#[repr(u32)]
pub enum CsaRepeatType {
    Minute = 1,
    Daily = 2,
    Weekly = 3,
    MonthlyPosition = 4,
    MonthlyDay = 5,
    YearlyMonth = 6,
    YearlyDay = 7,
}

/// Return the repeat type of the parsed rule.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn csa_recurrence_type(handle: *const CsaRecurrenceHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    let h = unsafe { &*handle };
    match h.rule.event {
        RepeatEvent::Minute { .. } => CsaRepeatType::Minute as u32,
        RepeatEvent::Daily { .. } => CsaRepeatType::Daily as u32,
        RepeatEvent::Weekly { .. } => CsaRepeatType::Weekly as u32,
        RepeatEvent::MonthlyByPos { .. } => CsaRepeatType::MonthlyPosition as u32,
        RepeatEvent::MonthlyByDay { .. } => CsaRepeatType::MonthlyDay as u32,
        RepeatEvent::YearlyByMonth { .. } => CsaRepeatType::YearlyMonth as u32,
        RepeatEvent::YearlyByDay { .. } => CsaRepeatType::YearlyDay as u32,
    }
}

/// Return the interval field (every-N) of the parsed rule, or 0 on error.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn csa_recurrence_interval(handle: *const CsaRecurrenceHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    let h = unsafe { &*handle };
    match h.rule.event {
        RepeatEvent::Minute { interval, .. }
        | RepeatEvent::Daily { interval, .. }
        | RepeatEvent::Weekly { interval, .. }
        | RepeatEvent::MonthlyByPos { interval, .. }
        | RepeatEvent::MonthlyByDay { interval, .. }
        | RepeatEvent::YearlyByMonth { interval, .. }
        | RepeatEvent::YearlyByDay { interval, .. } => interval,
    }
}

/// Return the duration as a signed int: >0 = finite count, 0 = infinite,
/// -1 = not set (matches the legacy `RE_NOTSET` / `RE_INFINITY` sentinels).
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn csa_recurrence_duration(handle: *const CsaRecurrenceHandle) -> i64 {
    if handle.is_null() {
        return -1;
    }
    let h = unsafe { &*handle };
    let d = match h.rule.event {
        RepeatEvent::Minute { duration, .. }
        | RepeatEvent::Daily { duration, .. }
        | RepeatEvent::Weekly { duration, .. }
        | RepeatEvent::MonthlyByPos { duration, .. }
        | RepeatEvent::MonthlyByDay { duration, .. }
        | RepeatEvent::YearlyByMonth { duration, .. }
        | RepeatEvent::YearlyByDay { duration, .. } => duration,
    };
    match d {
        Duration::NotSet => -1,
        Duration::Infinity => 0,
        Duration::Count(n) => n as i64,
    }
}
