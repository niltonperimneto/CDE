// Every unsafe operation inside an unsafe fn must be explicitly annotated.
#![deny(unsafe_op_in_unsafe_fn)]

pub mod format;
pub mod parser;
pub mod raima;
pub mod search;

use format::{DtSrHitword, DtSrKeytype, DtSrResult, DBLK, DB_ADDR, USRBLK};
use libc::{c_char, c_int, c_long};
use search::{QueryParser, Searcher};
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Initialisation state
//
// INITIALIZED gates all query operations: callers that skip DtSearchInit
// get a clear error code (DtSrFATAL = -1) rather than a silent no-op.
// DB_PATH stores the database path supplied at init time.
// ---------------------------------------------------------------------------
static INITIALIZED: AtomicBool = AtomicBool::new(false);
static DB_PATH: OnceLock<String> = OnceLock::new();

// DtSearch return codes (from DtSearch.h)
const DTSROK: c_int = 0;
const DTSRFATAL: c_int = -1;

// ---------------------------------------------------------------------------
// Public C API
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchInit(
    _argv0: *const c_char,
    config_file: *const c_char,
    _user_id: *const c_char,
    _init_flags: *const c_char,
    _user_data: *mut USRBLK,
) -> c_int {
    if INITIALIZED.load(Ordering::Acquire) {
        return DTSROK; // idempotent re-init
    }

    // Extract database path from config_file, if provided.
    if !config_file.is_null() {
        if let Ok(path) = unsafe { CStr::from_ptr(config_file) }.to_str() {
            // OnceLock::set fails silently if already set — that is fine here
            // because a second concurrent DtSearchInit is harmless.
            let _ = DB_PATH.set(path.to_owned());
        }
    }

    INITIALIZED.store(true, Ordering::Release);
    eprintln!(
        "[DtSearch] Initialised. DB path: {}",
        DB_PATH.get().map(String::as_str).unwrap_or("<none>")
    );
    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchReinit() -> c_int {
    // Clear state so DtSearchInit can run again.
    INITIALIZED.store(false, Ordering::Release);
    eprintln!("[DtSearch] Reinitialised");
    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchQuery(
    usrblk: *mut USRBLK,
    dbname: *const c_char,
    _search_type: c_int,
    _date1: *const c_char,
    _date2: *const c_char,
    _dittolist: *mut *mut DtSrResult,
    dittocount: *mut c_long,
    _stems: *mut c_char,
    _stemcount: *mut c_int,
) -> c_int {
    if !INITIALIZED.load(Ordering::Acquire) {
        eprintln!("[DtSearch] DtSearchQuery called before DtSearchInit");
        return DTSRFATAL;
    }

    // Resolve the database path: prefer the explicit dbname argument,
    // fall back to the path stored at init time.
    let db = if !dbname.is_null() {
        unsafe { CStr::from_ptr(dbname) }.to_str().ok().map(str::to_owned)
    } else {
        DB_PATH.get().cloned()
    };

    let db_path = match db {
        Some(p) => p,
        None => {
            eprintln!("[DtSearch] DtSearchQuery: no database path");
            return DTSRFATAL;
        }
    };

    // Extract the query string from usrblk.query (preferred) or usrblk itself.
    let query_str: Option<String> = if !usrblk.is_null() {
        let q_ptr = unsafe { (*usrblk).query };
        if !q_ptr.is_null() {
            unsafe { CStr::from_ptr(q_ptr) }.to_str().ok().map(str::to_owned)
        } else {
            None
        }
    } else {
        None
    };

    let query_str = match query_str {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("[DtSearch] DtSearchQuery: empty or missing query string");
            return DTSRFATAL;
        }
    };

    eprintln!("[DtSearch] Query='{}' db='{}'", query_str, db_path);

    // Parse the boolean query expression.
    let mut qp = QueryParser::new(&query_str);
    let query = match qp.parse() {
        Ok(q) => q,
        Err(e) => {
            eprintln!("[DtSearch] Parse error: {}", e);
            return DTSRFATAL;
        }
    };

    // Open the database, run the query, close on drop.
    let mut db_parser = crate::parser::DtSearchParser::new();
    if let Err(e) = db_parser.open(&db_path) {
        eprintln!("[DtSearch] Cannot open database '{}': {}", db_path, e);
        // Database unavailable is non-fatal for a stub build — return 0 hits.
        if !dittocount.is_null() {
            unsafe { *dittocount = 0 };
        }
        return DTSROK;
    }

    let searcher = Searcher::new();
    let results = searcher.search(&query);
    let count = results.len() as c_long;

    eprintln!("[DtSearch] Found {} results", count);

    if !dittocount.is_null() {
        unsafe { *dittocount = count };
    }

    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchGetKeytypes(
    _dbname: *const c_char,
    _ktcount: *mut c_int,
    _keytypes: *mut *mut DtSrKeytype,
) -> c_int {
    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchRetrieve(
    _dbname: *const c_char,
    _dba: DB_ADDR,
    _cleartext: *mut *mut c_char,
    _clearlen: *mut c_long,
    _fzkeyi: *mut c_int,
) -> c_int {
    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchHighlight(
    _dbname: *const c_char,
    _cleartext: *mut c_char,
    _hitwords: *mut *mut DtSrHitword,
    _hitwcount: *mut c_long,
    _search_type: c_int,
    _stems: *mut c_char,
    _stemcount: c_int,
) -> c_int {
    DTSROK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchFreeResults(_dittolist: *mut *mut DtSrResult) {
    // Results are not yet heap-allocated by Rust, so nothing to free.
}

/// Returns a pointer to a static C string.  The string is immutable and
/// lives for the process lifetime, so sharing it across threads is safe.
#[unsafe(no_mangle)]
pub extern "C" fn DtSearchGetMessages() -> *const c_char {
    b"No messages\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchAddMessage(msg: *const c_char) {
    if !msg.is_null() {
        if let Ok(s) = unsafe { CStr::from_ptr(msg) }.to_str() {
            eprintln!("[DtSearch] message: {}", s);
        }
    }
}

/// Reset search engine state.
///
/// S-4: The previous implementation called `std::process::exit` directly,
/// which skips all Rust destructors and C `atexit` handlers.  The caller
/// (C code) is responsible for calling `exit()`; we only clean up our own
/// state here.
#[unsafe(no_mangle)]
pub extern "C" fn DtSearchExit(_exit_code: c_int) {
    INITIALIZED.store(false, Ordering::Release);
    eprintln!("[DtSearch] DtSearchExit — engine shut down");
    // Do NOT call std::process::exit here.  Return to C; C will call exit().
}
