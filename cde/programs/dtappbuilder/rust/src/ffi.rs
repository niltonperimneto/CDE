//! C FFI for the BIL parser.
//!
//! Exposes an opaque-handle API that replaces the `bilP_load_*` callback
//! family driven by `bil_parse.y`. C callers can parse a `.bil` file and
//! introspect the resulting tree through stable handle functions, without
//! touching Rust memory directly.

use crate::ast::{Arg, BilFile};
use crate::parser::parse;
use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{self, AssertUnwindSafe};
use std::ptr;

// ---------------------------------------------------------------------------
// Opaque handle
// ---------------------------------------------------------------------------

/// Opaque handle to a parsed BIL file. Allocated on the Rust heap; freed
/// by [`cde_bil_free`].
pub struct BilHandle {
    pub(crate) file: BilFile,
    /// Interned NUL-terminated copies of strings we hand to C callers.
    /// Entries are append-only; existing pointers remain stable.
    strings: RefCell<Vec<CString>>,
}

// ---------------------------------------------------------------------------
// Thread-local last-error storage
// ---------------------------------------------------------------------------

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

/// Intern `s` as a NUL-terminated string stored on `handle`. The returned
/// pointer is valid for the lifetime of the handle.
fn intern(handle: &BilHandle, s: &str) -> *const c_char {
    let Ok(c) = CString::new(s) else {
        return ptr::null();
    };
    let mut strings = handle.strings.borrow_mut();
    strings.push(c);
    strings.last().unwrap().as_ptr()
}

/// Return the most recent parse error for this thread, or NULL if none.
///
/// # Safety
/// The returned pointer is owned by the thread-local; do **not** free it.
#[no_mangle]
pub extern "C" fn cde_bil_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(ptr::null())
    })
}

// ---------------------------------------------------------------------------
// Parse / free
// ---------------------------------------------------------------------------

/// Parse a BIL file given as a NUL-terminated UTF-8 string. Returns an
/// opaque handle on success, or NULL on failure — call [`cde_bil_last_error`]
/// to retrieve the diagnostic.
///
/// # Safety
/// `input` must be a NUL-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_parse(input: *const c_char) -> *mut BilHandle {
    clear_last_error();
    if input.is_null() {
        set_last_error("cde_bil_parse: NULL input");
        return ptr::null_mut();
    }
    let res = panic::catch_unwind(AssertUnwindSafe(|| {
        let s = match unsafe { CStr::from_ptr(input) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("cde_bil_parse: input not UTF-8");
                return ptr::null_mut();
            }
        };
        match parse(s) {
            Ok(file) => Box::into_raw(Box::new(BilHandle {
                file,
                strings: RefCell::new(Vec::new()),
            })),
            Err(e) => {
                set_last_error(format!("{e}"));
                ptr::null_mut()
            }
        }
    }));
    match res {
        Ok(p) => p,
        Err(_) => {
            set_last_error("cde_bil_parse: Rust panic");
            ptr::null_mut()
        }
    }
}

/// Free a handle previously returned by [`cde_bil_parse`].
///
/// # Safety
/// `handle` must be NULL or a pointer previously returned by [`cde_bil_parse`].
#[no_mangle]
pub unsafe extern "C" fn cde_bil_free(handle: *mut BilHandle) {
    if handle.is_null() {
        return;
    }
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        drop(unsafe { Box::from_raw(handle) });
    }));
}

// ---------------------------------------------------------------------------
// Introspection helpers — all wrapped in catch_unwind per RUST_MIGRATION_PLAN §3.3
// ---------------------------------------------------------------------------

/// Return the major version from `:bil-version`, or 0 if absent or on error.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_version_major(handle: *const BilHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    panic::catch_unwind(AssertUnwindSafe(|| {
        unsafe { &*handle }.file.version.map(|(maj, _)| maj).unwrap_or(0)
    }))
    .unwrap_or_else(|_| { set_last_error("cde_bil_version_major: Rust panic"); 0 })
}

/// Return the minor version from `:bil-version`, or 0 if absent or on error.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_version_minor(handle: *const BilHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    panic::catch_unwind(AssertUnwindSafe(|| {
        unsafe { &*handle }.file.version.map(|(_, min)| min).unwrap_or(0)
    }))
    .unwrap_or_else(|_| { set_last_error("cde_bil_version_minor: Rust panic"); 0 })
}

/// Return the number of top-level items in the file, or 0 on error.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_count(handle: *const BilHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    panic::catch_unwind(AssertUnwindSafe(|| {
        unsafe { &*handle }.file.items.len()
    }))
    .unwrap_or_else(|_| { set_last_error("cde_bil_item_count: Rust panic"); 0 })
}

/// Return the keyword of the `idx`-th top-level item as a NUL-terminated
/// interned string, or NULL if `idx` is out of range or on error.
///
/// # Safety
/// `handle` must be valid. The returned pointer lives as long as the handle.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_keyword(
    handle: *const BilHandle,
    idx: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    panic::catch_unwind(AssertUnwindSafe(|| {
        let h = unsafe { &*handle };
        match h.file.items.get(idx) {
            Some(item) => intern(h, &item.keyword),
            None => ptr::null(),
        }
    }))
    .unwrap_or_else(|_| { set_last_error("cde_bil_item_keyword: Rust panic"); ptr::null() })
}

/// Return the name (first atom arg) of the `idx`-th top-level item as a
/// NUL-terminated interned string, or NULL if none or on error.
///
/// # Safety
/// `handle` must be valid. The returned pointer lives as long as the handle.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_name(
    handle: *const BilHandle,
    idx: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    panic::catch_unwind(AssertUnwindSafe(|| {
        let h = unsafe { &*handle };
        let item = match h.file.items.get(idx) {
            Some(i) => i,
            None => return ptr::null(),
        };
        match item.args.first() {
            Some(Arg::Atom(s)) => intern(h, s),
            _ => ptr::null(),
        }
    }))
    .unwrap_or_else(|_| { set_last_error("cde_bil_item_name: Rust panic"); ptr::null() })
}
