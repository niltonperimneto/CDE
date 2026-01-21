pub mod format;
pub mod parser;
pub mod raima;
pub mod search;

use format::{DtSrHitword, DtSrKeytype, DtSrResult, DBLK, DB_ADDR, USRBLK};
use libc::{c_char, c_int, c_long, c_void};
use search::{QueryParser, Searcher};
use std::ffi::CStr;

#[unsafe(no_mangle)]
pub extern "C" fn RsBooleanSearch(query_str: *const c_char) -> c_int {
    if query_str.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(query_str) };
    let q_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    println!("Rust: Executing Search Query: {}", q_str);

    let mut qp = QueryParser::new(q_str);
    let query = match qp.parse() {
        Ok(q) => q,
        Err(e) => {
            println!("Rust: Parse Error: {}", e);
            return -1;
        }
    };

    let searcher = Searcher::new();
    let results = searcher.search(&query);
    println!("Rust: Found {} results", results.len());

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchInit(
    _argv0: *const c_char,
    _config_file: *const c_char,
    _user_id: *const c_char,
    _init_flags: *const c_char,
    _user_data: *mut USRBLK,
) -> c_int {
    println!("Rust DtSearchInit called!");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchReinit() -> c_int {
    println!("Rust DtSearchReinit called!");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchQuery(
    usrblk: *mut USRBLK,
    _dbname: *const c_char,
    _search_type: c_int,
    _date1: *const c_char,
    _date2: *const c_char,
    _dittolist: *mut *mut DtSrResult,
    _dittocount: *mut c_long,
    _stems: *mut c_char,
    _stemcount: *mut c_int,
) -> c_int {
    unsafe {
        if !usrblk.is_null() {
            println!(
                "Rust DtSearchQuery called from user: {:?}",
                (*usrblk).userid
            );
        }
    }
    0 // DtSrOK
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchGetKeytypes(
    _dbname: *const c_char,
    _ktcount: *mut c_int,
    _keytypes: *mut *mut DtSrKeytype,
) -> c_int {
    println!("Rust DtSearchGetKeytypes called");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchRetrieve(
    _dbname: *const c_char,
    _dba: DB_ADDR,
    _cleartext: *mut *mut c_char,
    _clearlen: *mut c_long,
    _fzkeyi: *mut c_int,
) -> c_int {
    println!("Rust DtSearchRetrieve called");
    0
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
    println!("Rust DtSearchHighlight called");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchFreeResults(_dittolist: *mut *mut DtSrResult) {
    println!("Rust DtSearchFreeResults called");
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchGetMessages() -> *const c_char {
    // static string
    b"No messages\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchAddMessage(_msg: *const c_char) {
    println!("Rust DtSearchAddMessage called");
}

#[unsafe(no_mangle)]
pub extern "C" fn DtSearchExit(_exit_code: c_int) {
    println!("Rust DtSearchExit called");
    std::process::exit(_exit_code);
}
