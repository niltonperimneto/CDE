//! C FFI layer.
//!
//! Exposes an opaque `CdeDtHandle` plus getter functions. All allocations
//! live on the Rust side; the C caller never dereferences Rust structs
//! directly. Every entry point catches Rust panics so an unwind never
//! crosses the FFI boundary (per `RUST_MIGRATION_PLAN.md` §3.3).

use crate::ast::Database;
use crate::loader;
use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use std::ptr;

/// Opaque handle returned to C. Consumers pass this back in on every call.
pub struct CdeDtHandle {
    db: Database,
    // Owned C strings handed out via getters; cleared on Drop.
    strings: RefCell<Vec<CString>>,
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: impl Into<String>) {
    let c = CString::new(msg.into()).unwrap_or_else(|_| CString::new("<invalid error>").unwrap());
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(c));
}

fn clear_last_error() {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}

/// Return the most recent error message from this thread, or NULL if the
/// previous call succeeded. The returned pointer remains valid until the
/// next FFI call on this thread.
///
/// # Safety
/// The caller must not free the returned pointer.
#[no_mangle]
pub extern "C" fn cde_dt_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(ptr::null())
    })
}

/// Parse every `.dt` file under `path` in parallel.
///
/// # Safety
/// `path` must point to a NUL-terminated UTF-8 string. Returns NULL on
/// failure; call [`cde_dt_last_error`] to retrieve the message.
#[no_mangle]
pub unsafe extern "C" fn cde_dt_load_dir(path: *const c_char) -> *mut CdeDtHandle {
    clear_last_error();
    if path.is_null() {
        set_last_error("cde_dt_load_dir: NULL path");
        return ptr::null_mut();
    }
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let c = unsafe { CStr::from_ptr(path) };
        let s = match c.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("cde_dt_load_dir: path is not valid UTF-8");
                return ptr::null_mut();
            }
        };
        let db = loader::load_dir(std::path::Path::new(s));
        Box::into_raw(Box::new(CdeDtHandle {
            db,
            strings: RefCell::new(Vec::new()),
        }))
    }));
    match result {
        Ok(p) => p,
        Err(_) => {
            set_last_error("cde_dt_load_dir: Rust panic");
            ptr::null_mut()
        }
    }
}

/// Free a handle previously returned by [`cde_dt_load_dir`].
///
/// # Safety
/// `handle` must be NULL or a pointer previously returned by
/// [`cde_dt_load_dir`].
#[no_mangle]
pub unsafe extern "C" fn cde_dt_free(handle: *mut CdeDtHandle) {
    if handle.is_null() {
        return;
    }
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        drop(unsafe { Box::from_raw(handle) });
    }));
}

/// Number of ACTION records loaded into the handle.
#[no_mangle]
pub unsafe extern "C" fn cde_dt_action_count(handle: *const CdeDtHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).db.action_count() }
}

/// Number of DATA_ATTRIBUTES records loaded into the handle.
#[no_mangle]
pub unsafe extern "C" fn cde_dt_datatype_count(handle: *const CdeDtHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).db.datatype_count() }
}

fn intern_cstring(handle: &CdeDtHandle, s: &str) -> *const c_char {
    let c = match CString::new(s) {
        Ok(c) => c,
        Err(_) => return ptr::null(),
    };
    let mut strings = handle.strings.borrow_mut();
    strings.push(c);
    // SAFETY: the entry we just pushed outlives the borrow of `strings`
    // because the Vec is owned by `handle` which outlives this call.
    strings.last().unwrap().as_ptr()
}

/// Look up an ACTION by name and return its EXEC_STRING field, or NULL if
/// the action or the field is missing. The returned pointer is owned by the
/// handle and remains valid until the handle is freed.
///
/// # Safety
/// `handle` must be a valid pointer from [`cde_dt_load_dir`] and `name`
/// must be a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn cde_dt_action_get_exec(
    handle: *const CdeDtHandle,
    name: *const c_char,
) -> *const c_char {
    clear_last_error();
    if handle.is_null() || name.is_null() {
        return ptr::null();
    }
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let h = unsafe { &*handle };
        let s = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null(),
        };
        let exec = h.db.actions.get(s).and_then(|r| r.exec_string());
        match exec {
            Some(value) => intern_cstring(h, value),
            None => ptr::null(),
        }
    }));
    result.unwrap_or(ptr::null())
}

/// Look up an ACTION's LABEL field.
///
/// # Safety
/// See [`cde_dt_action_get_exec`].
#[no_mangle]
pub unsafe extern "C" fn cde_dt_action_get_label(
    handle: *const CdeDtHandle,
    name: *const c_char,
) -> *const c_char {
    if handle.is_null() || name.is_null() {
        return ptr::null();
    }
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let h = unsafe { &*handle };
        let s = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null(),
        };
        let label = h.db.actions.get(s).and_then(|r| r.label());
        match label {
            Some(value) => intern_cstring(h, value),
            None => ptr::null(),
        }
    }));
    result.unwrap_or(ptr::null())
}

/// Parse a single `.dt` file into an already-existing handle. Useful for
/// reloading after a user edits a config file.
///
/// # Safety
/// Both pointers must be valid. `file` must be NUL-terminated UTF-8.
/// Returns 0 on success, -1 on failure.
#[no_mangle]
pub unsafe extern "C" fn cde_dt_load_file(
    handle: *mut CdeDtHandle,
    file: *const c_char,
) -> i32 {
    clear_last_error();
    if handle.is_null() || file.is_null() {
        set_last_error("cde_dt_load_file: NULL argument");
        return -1;
    }
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let h = unsafe { &mut *handle };
        let s = match unsafe { CStr::from_ptr(file) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("cde_dt_load_file: path is not valid UTF-8");
                return -1;
            }
        };
        match crate::parser::parse_file(PathBuf::from(s).as_path()) {
            Ok(records) => {
                for r in records {
                    h.db.insert(r);
                }
                0
            }
            Err(e) => {
                set_last_error(format!("{e:?}"));
                -1
            }
        }
    }));
    result.unwrap_or_else(|_| {
        set_last_error("cde_dt_load_file: Rust panic");
        -1
    })
}
