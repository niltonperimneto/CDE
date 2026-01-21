use libc::{c_char, c_int, c_long, c_void};

// Status codes from vista.h
pub const S_OKAY: c_int = 0;
pub const S_DBOPEN: c_int = -1;
pub const S_INVDB: c_int = -4;

// Constants from vista.h
pub const DB_ADDR: usize = 4; // size of DB_ADDR which is LONG (32-bit typically)

extern "C" {
    pub fn d_open(name: *const c_char, mode: *const c_char) -> c_int;
    pub fn d_close() -> c_int;
    pub fn d_setpages(pages: c_int, size: c_int) -> c_int;
    pub fn d_recfrst(rectype: c_int, lock: c_int) -> c_int;
    pub fn d_recread(buf: *mut c_void, lock: c_int) -> c_int;
    pub fn d_keyfind(rectype: c_int, key: *const c_char, lock: c_int) -> c_int;
    pub fn d_keynext(rectype: c_int, lock: c_int) -> c_int;
    pub fn d_keyread(buf: *mut c_char) -> c_int;
    pub fn d_dblog(name: *const c_char) -> c_int;
}
