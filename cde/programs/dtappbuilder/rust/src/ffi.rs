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
            Ok(file) => Box::into_raw(Box::new(BilHandle { file })),
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
// Introspection helpers
// ---------------------------------------------------------------------------

/// Return the major version from `:bil-version`, or 0 if absent.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_version_major(handle: *const BilHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    unsafe { &*handle }.file.version.map(|(maj, _)| maj).unwrap_or(0)
}

/// Return the minor version from `:bil-version`, or 0 if absent.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_version_minor(handle: *const BilHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }
    unsafe { &*handle }.file.version.map(|(_, min)| min).unwrap_or(0)
}

/// Return the number of top-level items in the file.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_count(handle: *const BilHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { &*handle }.file.items.len()
}

/// Return the keyword of the `idx`-th top-level item as a NUL-terminated
/// string, or NULL if `idx` is out of range.
///
/// # Safety
/// `handle` must be valid. Returned pointer is borrowed from the handle and
/// must not outlive it.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_keyword(
    handle: *const BilHandle,
    idx: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    let file = &unsafe { &*handle }.file;
    match file.items.get(idx) {
        Some(item) => item.keyword.as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

/// Return the name (first atom arg) of the `idx`-th top-level item, or NULL
/// if the item has no atom first argument.
///
/// # Safety
/// `handle` must be valid. Returned pointer is borrowed from the handle.
#[no_mangle]
pub unsafe extern "C" fn cde_bil_item_name(
    handle: *const BilHandle,
    idx: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    let file = &unsafe { &*handle }.file;
    let item = match file.items.get(idx) {
        Some(i) => i,
        None => return ptr::null(),
    };
    match item.args.first() {
        Some(Arg::Atom(s)) => s.as_ptr() as *const c_char,
        _ => ptr::null(),
    }
}
