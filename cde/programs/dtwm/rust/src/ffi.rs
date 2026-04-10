//! C FFI for dtwm's dtwmrc parser.

use crate::ast::ResourceFile;
use crate::parser;
use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::ptr;

pub struct CdeDtwmHandle {
    file: ResourceFile,
    strings: RefCell<Vec<CString>>,
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

/// Return the most recent parse error on this thread, or NULL.
///
/// # Safety
/// The returned pointer is owned by the thread-local; do not free it.
#[no_mangle]
pub extern "C" fn cde_dtwm_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(ptr::null())
    })
}

/// Parse a dtwmrc file from disk.
///
/// # Safety
/// `path` must be a NUL-terminated UTF-8 string. Returns NULL on failure.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_parse_file(path: *const c_char) -> *mut CdeDtwmHandle {
    clear_last_error();
    if path.is_null() {
        set_last_error("cde_dtwm_parse_file: NULL path");
        return ptr::null_mut();
    }
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let s = match unsafe { CStr::from_ptr(path) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("cde_dtwm_parse_file: path not UTF-8");
                return ptr::null_mut();
            }
        };
        match parser::parse_file(Path::new(s)) {
            Ok(file) => Box::into_raw(Box::new(CdeDtwmHandle {
                file,
                strings: RefCell::new(Vec::new()),
            })),
            Err(e) => {
                set_last_error(format!("{e:?}"));
                ptr::null_mut()
            }
        }
    }));
    match result {
        Ok(p) => p,
        Err(_) => {
            set_last_error("cde_dtwm_parse_file: Rust panic");
            ptr::null_mut()
        }
    }
}

/// Free a handle previously returned by [`cde_dtwm_parse_file`].
///
/// # Safety
/// `handle` must be NULL or a pointer previously returned here.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_free(handle: *mut CdeDtwmHandle) {
    if handle.is_null() {
        return;
    }
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        drop(unsafe { Box::from_raw(handle) });
    }));
}

/// Number of top-level declarations parsed from the file.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_decl_count(handle: *const CdeDtwmHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).file.declarations.len() }
}

/// Return the declaration kind as a static string, or NULL on out-of-range.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_decl_kind(
    handle: *const CdeDtwmHandle,
    index: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    let decls = unsafe { &(*handle).file.declarations };
    let Some(decl) = decls.get(index) else {
        return ptr::null();
    };
    // Static strings, safe to return directly.
    match decl.kind {
        crate::ast::DeclKind::Menu => b"Menu\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Keys => b"Keys\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Buttons => b"Buttons\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Panel => b"PANEL\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Box_ => b"BOX\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Control => b"CONTROL\0".as_ptr() as *const c_char,
        crate::ast::DeclKind::Switch => b"SWITCH\0".as_ptr() as *const c_char,
    }
}

fn intern_cstring(handle: &CdeDtwmHandle, s: &str) -> *const c_char {
    let Ok(c) = CString::new(s) else {
        return ptr::null();
    };
    let mut strings = handle.strings.borrow_mut();
    strings.push(c);
    strings.last().unwrap().as_ptr()
}

/// Return the declaration name (interned for the handle's lifetime).
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_decl_name(
    handle: *const CdeDtwmHandle,
    index: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    let h = unsafe { &*handle };
    let Some(decl) = h.file.declarations.get(index) else {
        return ptr::null();
    };
    intern_cstring(h, &decl.name)
}

/// Return the declaration body text (interned for the handle's lifetime).
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn cde_dtwm_decl_body(
    handle: *const CdeDtwmHandle,
    index: usize,
) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    let h = unsafe { &*handle };
    let Some(decl) = h.file.declarations.get(index) else {
        return ptr::null();
    };
    intern_cstring(h, &decl.body)
}
