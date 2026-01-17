use libc::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

// Basic Types (Mapping to CDE's Tt_status, Tt_mode, etc.)
pub type TtStatus = c_int;
pub type TtMode = c_int;
pub type TtScope = c_int;
pub type TtClass = c_int;
pub type TtCategory = c_int;
pub type TtCallback = extern "C" fn(*mut c_void, *mut c_void) -> TtStatus; // Simplified

// Constants (These need to match tt_c.h values accurately eventually)
pub const TT_OK: TtStatus = 0;
pub const TT_WRN_NOTFOUND: TtStatus = 1; // Placeholder

#[repr(transparent)]
pub struct SyncConstPtr(pub *const c_char);
unsafe impl Sync for SyncConstPtr {}

#[no_mangle]
pub static Tttk_string: SyncConstPtr = SyncConstPtr(b"string\0".as_ptr() as *const c_char);

#[no_mangle]
pub static Tttk_message_id: SyncConstPtr = SyncConstPtr(b"messageID\0".as_ptr() as *const c_char);

// ... existing code ...

use once_cell::sync::{Lazy, OnceCell};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use zbus::blocking::Connection;

// Global queue for incoming messages
static MSG_QUEUE: Lazy<Mutex<VecDeque<TtMessage>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

// Pipe FDs for waking up the main loop
static PIPE_READ: AtomicI32 = AtomicI32::new(-1);
static PIPE_WRITE: AtomicI32 = AtomicI32::new(-1);

static CONN: OnceCell<Connection> = OnceCell::new();

#[no_mangle]
pub extern "C" fn tt_initialize() -> TtStatus {
    eprintln!("[libtt_shim] tt_initialize called");
    if CONN.get().is_some() {
        return TT_OK;
    }

    // Create a pipe for event loop integration
    let mut fds: [c_int; 2] = [-1, -1];
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            eprintln!("[libtt_shim] Failed to create pipe");
            return TT_WRN_NOTFOUND;
        }
    }
    PIPE_READ.store(fds[0], Ordering::SeqCst);
    PIPE_WRITE.store(fds[1], Ordering::SeqCst);

    match Connection::session() {
        Ok(conn) => {
            // Clone connection for the background thread *before* setting the global one
            // zbus::blocking::Connection is Clone
            let thread_conn = conn.clone();

            thread::spawn(move || {
                listen_loop(thread_conn);
            });

            if let Err(_) = CONN.set(conn) {
                eprintln!("[libtt_shim] Failed to set global connection");
                return TT_WRN_NOTFOUND;
            }
            eprintln!("[libtt_shim] D-Bus session connection established");
            TT_OK
        }
        Err(e) => {
            eprintln!("[libtt_shim] Failed to connect to session bus: {}", e);
            TT_WRN_NOTFOUND
        }
    }
}

// --- Event Loop Implementation ---

fn listen_loop(conn: Connection) {
    eprintln!("[libtt_shim] Background thread started");

    // Use MessageIterator for blocking iteration (zbus 4.x style)
    let mut iter = zbus::blocking::MessageIterator::from(&conn);

    // Iterate over incoming D-Bus messages blocking
    for msg in iter {
        match msg {
            Ok(m) => {
                // Ignore our own signals if possible, or filter.
                // For now, simpler stub: op = member name
                // In a real impl, we'd parse the body recursively as TtMsg args
                // Access member via header() in zbus 3.x+
                let header = m.header();
                let member = header.member();

                // Explicitly handle the Option<&MemberName> to string conversion
                // using if let to guide type inference
                let op = if let Some(name) = member {
                    name.as_str().to_string()
                } else {
                    String::new()
                };

                eprintln!("[libtt_shim] Received D-Bus message: Op='{}'", op);

                let tt_msg = TtMessage {
                    op,
                    // ... defaults
                    ..TtMessage::new()
                };

                // Push to queue
                if let Ok(mut q) = MSG_QUEUE.lock() {
                    q.push_back(tt_msg);
                }

                // Wake up main thread
                let pipe_write_fd = PIPE_WRITE.load(Ordering::SeqCst);
                if pipe_write_fd != -1 {
                    unsafe {
                        let buf = b"!";
                        libc::write(pipe_write_fd, buf.as_ptr() as *const c_void, 1);
                    }
                }
            }
            Err(e) => {
                eprintln!("[libtt_shim] D-Bus error: {}", e);
                // Avoid tight loop usage on error
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tt_default_session() -> *mut c_char {
    eprintln!("[libtt_shim] tt_default_session called");
    // Return a dummy session ID
    CString::new("01 12345 127.0.0.1 1000 1000 /tmp/.TT_SESSION")
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn tt_open() -> *mut c_char {
    eprintln!("[libtt_shim] tt_open called");
    // Return a dummy procid
    CString::new("12345").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tt_close() -> TtStatus {
    eprintln!("[libtt_shim] tt_close called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_fd() -> c_int {
    PIPE_READ.load(Ordering::SeqCst)
}

// --- Memory Management ---

#[no_mangle]
pub extern "C" fn tt_free(p: *mut c_void) {
    // In real implementation, we need to know if this was allocated by Rust (CString) or C.
    // shim implementation of tt_default_session returns CString::into_raw, so strict tt_free would need to reclaim it.
    // For now, specific leak is better than double-free crash.
    if !p.is_null() {
        unsafe {
            // let _ = CString::from_raw(p as *mut c_char); // Uncomment if we track allocations accurately
        }
    }
}

#[no_mangle]
pub extern "C" fn tt_ptr_error(_p: *const c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pointer_error(_p: *const c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_mark() -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn tt_release(p: *mut c_void) {
    tt_free(p);
}

#[no_mangle]
pub extern "C" fn tt_is_err(s: TtStatus) -> c_int {
    if s != TT_OK {
        1
    } else {
        0
    }
}

// --- Message Handling ---

mod message;
use message::TtMessage;

// ... (keep constants)

// --- Message Handling ---

#[no_mangle]
pub extern "C" fn tt_message_create() -> *mut c_void {
    // Allocate a new TtMessage on the heap and return raw pointer
    let msg = Box::new(TtMessage::new());
    Box::into_raw(msg) as *mut c_void
}

#[no_mangle]
pub extern "C" fn tt_message_reply(_m: *mut c_void) -> TtStatus {
    eprintln!("[libtt_shim] tt_message_reply called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_val(_m: *mut c_void, _n: c_int) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_destroy(m: *mut c_void) -> TtStatus {
    if !m.is_null() {
        unsafe {
            // Reconstruct Box to drop it
            let _ = Box::from_raw(m as *mut TtMessage);
        }
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_send(m: *mut c_void) -> TtStatus {
    if show_trace() {
        eprintln!("[libtt_shim] tt_message_send called");
    }

    if m.is_null() {
        return TT_WRN_NOTFOUND;
    }

    let msg = unsafe { &*(m as *mut TtMessage) };

    // Check if we have a connection
    if let Some(conn) = CONN.get() {
        // Send via D-Bus
        // Mapping strategy:
        // Interface: "org.cde.ToolTalk"
        // Path: "/org/cde/ToolTalk"
        // Member: msg.op

        // This is a simplified example. We'd likely need to serialize args into the body.
        // For now, just logging the intent is a massive step up from silent no-op.
        eprintln!(
            "[libtt_shim] Sending D-Bus Signal: Op='{}', Args={:?}",
            msg.op, msg.args
        );

        let res = conn.emit_signal(
            Option::<&str>::None, // Destination (broadcast)
            "/org/cde/ToolTalk",
            "org.cde.ToolTalk",
            msg.op.as_str(),
            // We need to implement Body serialization for TtArg explicitly or pass empty for now to test connection
            &(),
        );

        if let Err(e) = res {
            eprintln!("[libtt_shim] D-Bus Error: {}", e);
        }
    } else {
        eprintln!("[libtt_shim] No D-Bus connection available!");
    }

    TT_OK
}

// ...

#[no_mangle]
pub extern "C" fn tt_message_op_set(m: *mut c_void, op: *const c_char) -> TtStatus {
    if !m.is_null() {
        let msg = unsafe { &mut *(m as *mut TtMessage) };
        msg.set_op(op);
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_add(
    m: *mut c_void,
    mode: TtMode,
    vtype: *const c_char,
    value: *const c_char,
) -> TtStatus {
    if !m.is_null() {
        let msg = unsafe { &mut *(m as *mut TtMessage) };
        msg.add_arg(mode, vtype, value);
    }
    TT_OK
}

fn show_trace() -> bool {
    std::env::var("TT_TRACE_SCRIPT").is_ok()
}

#[no_mangle]
pub extern "C" fn tt_message_op(_m: *mut c_void) -> *mut c_char {
    CString::new("").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tt_message_file(_m: *mut c_void) -> *mut c_char {
    ptr::null_mut()
}

mod pattern;
use pattern::TtPattern;

// Global pattern registry
static PATTERNS: OnceCell<Mutex<Vec<Box<TtPattern>>>> = OnceCell::new();

#[no_mangle]
pub extern "C" fn tt_pattern_create() -> *mut c_void {
    let pat = Box::new(TtPattern::new());
    Box::into_raw(pat) as *mut c_void
}

#[no_mangle]
pub extern "C" fn tt_pattern_destroy(p: *mut c_void) -> TtStatus {
    if !p.is_null() {
        unsafe {
            let _ = Box::from_raw(p as *mut TtPattern);
        }
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_register(p: *mut c_void) -> TtStatus {
    if p.is_null() {
        return TT_WRN_NOTFOUND;
    }

    // We clone the pattern logic or just move ownership?
    // Usually tt_pattern_register implicitly keeps the pattern active.
    // But the C API allows the user to hold the pointer 'p'.
    // So we should probably register a reference or identifying handle.
    // For this shim, let's just assert validity.
    // In a real impl, we'd add it to PATTERNS.

    let patterns_lock = PATTERNS.get_or_init(|| Mutex::new(Vec::new()));
    if let Ok(mut patterns) = patterns_lock.lock() {
        // This is tricky because `p` is owned by the C side until destroy.
        // We can't take ownership. We store a raw pointer? Unsafe but necessary.
        // OR we don't store it yet but just acknowledge.
        eprintln!("[libtt_shim] Registered pattern {:?}", p);
    }

    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_unregister(p: *mut c_void) -> TtStatus {
    eprintln!("[libtt_shim] Unregistered pattern {:?}", p);
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_category_set(p: *mut c_void, c: TtCategory) -> TtStatus {
    if !p.is_null() {
        let pat = unsafe { &mut *(p as *mut TtPattern) };
        pat.category = c;
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_scope_add(p: *mut c_void, s: TtScope) -> TtStatus {
    if !p.is_null() {
        let pat = unsafe { &mut *(p as *mut TtPattern) };
        pat.add_scope(s);
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_op_add(p: *mut c_void, op: *const c_char) -> TtStatus {
    if !p.is_null() {
        let pat = unsafe { &mut *(p as *mut TtPattern) };
        pat.add_op(op);
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_callback_add(p: *mut c_void, cb: TtCallback) -> TtStatus {
    if !p.is_null() {
        let pat = unsafe { &mut *(p as *mut TtPattern) };
        pat.add_callback(cb);
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_host_file_netfile(_host: *const c_char, _file: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_netfile_file(_netfile: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_class_set(_m: *mut c_void, _c: TtClass) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_scope_set(_m: *mut c_void, _s: TtScope) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_address_set(_m: *mut c_void, _a: c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_session_set(_m: *mut c_void, _s: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_file_set(m: *mut c_void, f: *const c_char) -> TtStatus {
    if !m.is_null() {
        let msg = unsafe { &mut *(m as *mut TtMessage) };
        msg.set_file(f);
    }
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_default_procid() -> *mut c_char {
    CString::new("12345").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tt_int_error(_i: c_int) -> TtStatus {
    TT_OK
}

fn dispatch_message(msg: TtMessage) {
    // Find matching patterns
    let patterns_lock = PATTERNS.get_or_init(|| Mutex::new(Vec::new()));
    if let Ok(patterns) = patterns_lock.lock() {
        for pat in patterns.iter() {
            // Simplified matching logic: just match op if present in pattern
            if pat.ops.is_empty() || pat.ops.contains(&msg.op) {
                // Match! Call callbacks
                for cb in &pat.callbacks {
                    eprintln!("[libtt_shim] Dispatching message '{}' to callback", msg.op);
                    // We need to create a pointer for msg to pass to C
                    // NOTE: TtCallback signature is (m, p) or similar.
                    // Ideally we pass a valid TtMessage pointer.
                    // For now, we leak it or use a stack one?
                    // Callbacks expect to own or borrow the message?
                    // Usually they reply or destroy it.
                    let m_ptr = Box::into_raw(Box::new(msg.clone())) as *mut c_void;

                    // The second arg is pattern, or client data?
                    // TtCallback: extern "C" fn(*mut c_void, *mut c_void) -> TtStatus;
                    // Usually (msg, pattern)
                    // We'll pass null for pattern or the current one (hard to pass reference to C from here safely?)
                    let p_ptr = ptr::null_mut();

                    cb(m_ptr, p_ptr);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tttk_Xt_input_handler(_p: *mut c_void, _s: *mut c_int, _id: *mut c_int) {
    // 1. Drain pipe
    let fd = PIPE_READ.load(Ordering::SeqCst);
    if fd != -1 {
        let mut buf = [0u8; 16];
        unsafe { libc::read(fd, buf.as_mut_ptr() as *mut c_void, 16) };
    }

    // 2. Process Queue
    if let Ok(mut q) = MSG_QUEUE.lock() {
        while let Some(msg) = q.pop_front() {
            dispatch_message(msg);
        }
    }
}

#[no_mangle]
pub extern "C" fn tt_pattern_state_add(_p: *mut c_void, _s: c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_session_add(_p: *mut c_void, _s: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_arg_add(
    _p: *mut c_void,
    _m: TtMode,
    _v: *const c_char,
    _val: *const c_char,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_user_set(_p: *mut c_void, _u: c_int, _v: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_state(_m: *mut c_void) -> c_int {
    3 // TT_CREATED or similar
}

#[no_mangle]
pub extern "C" fn tttk_message_destroy(_m: *mut c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pnotice_create(_scope: TtScope, _op: *const c_char) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_file_netfile(_m: *mut c_void, _f: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_malloc(s: usize) -> *mut c_void {
    unsafe { libc::malloc(s) }
}

#[no_mangle]
pub extern "C" fn tt_host_netfile_file(_h: *const c_char, _n: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_sender(_m: *mut c_void) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_user(_m: *mut c_void, _i: c_int, _key: *const c_char) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_status_message(_s: TtStatus) -> *mut c_char {
    CString::new("Success").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tttk_message_create(
    _m: *mut c_void,
    _c: TtClass,
    _s: TtScope,
    _handler: *const c_char,
    _op: *const c_char,
    _callback: TtCallback,
) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_callback_add(_m: *mut c_void, _cb: TtCallback) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_pattern_user(_p: *mut c_void, _i: c_int, _key: *const c_char) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_pattern_class_add(_p: *mut c_void, _c: TtClass) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_ival(_m: *mut c_void, _n: c_int, _val: *mut c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_ival_set(_m: *mut c_void, _n: c_int, _val: c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_status(_m: *mut c_void) -> c_int {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_status_string(_m: *mut c_void) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_arg_type(_m: *mut c_void, _n: c_int) -> TtMode {
    0 // TT_IN/OUT/etc
}

#[no_mangle]
pub extern "C" fn tt_message_args_count(_m: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn tt_message_arg_bval(
    _m: *mut c_void,
    _n: c_int,
    _val: *mut *mut c_uchar,
    _len: *mut c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tttk_message_abandon(_m: *mut c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tttk_string_op(_op: *const c_char) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn ttmedia_load(
    _m: *mut c_void,
    _cb: TtCallback,
    _clientdata: *mut c_void,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_user_set(_m: *mut c_void, _u: c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttdt_subcontract_manage(
    _m: *mut c_void,
    _cb: TtCallback,
    _prop: *const c_char,
    _clientdata: *mut c_void,
) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_iarg_add(
    _m: *mut c_void,
    _mode: TtMode,
    _type: *const c_char,
    _val: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_barg_add(
    _m: *mut c_void,
    _mode: TtMode,
    _type: *const c_char,
    _val: *const c_uchar,
    _len: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_bval_set(
    _m: *mut c_void,
    _n: c_int,
    _val: *const c_uchar,
    _len: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_fail(_m: *mut c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tttk_message_fail(
    _m: *mut c_void,
    _s: TtStatus,
    _p: *const c_char,
    _i: c_int,
) -> TtStatus {
    TT_OK
}

use libc::c_uchar;

// --- Missing ToolTalk Desktop Services (ttdt_*) Stubs ---

#[no_mangle]
pub extern "C" fn ttdt_file_join(
    _f: *const c_char,
    _scope: TtScope,
    _join: c_int,
    _cb: TtCallback,
    _c_data: *mut c_void,
) -> *mut c_void {
    eprintln!("[libtt_shim] ttdt_file_join stub called");
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ttdt_message_accept(
    _m: *mut c_void,
    _cb: TtCallback,
    _c_data: *mut c_void,
    _acc: *mut c_void,
    _acc_cb: TtCallback,
    _acc_c_data: *mut c_void,
) -> *mut c_void {
    // Returns a pattern or status? It seems to return implicitly a created pattern handle or similar.
    // Assuming return type is TtPattern derived or void*.
    // From man pages: Tt_pattern ttdt_message_accept(...)
    // TtPattern is likely a pointer in C (Tt_pattern).
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ttdt_file_quit(_patterns: *mut *mut c_void, _quit: c_int) -> TtStatus {
    eprintln!("[libtt_shim] ttdt_file_quit stub called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttdt_session_quit(
    _sess: *const c_char,
    _patterns: *mut *mut c_void,
    _quit: c_int,
) -> TtStatus {
    eprintln!("[libtt_shim] ttdt_session_quit stub called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttdt_close(
    _procid: *const c_char,
    _new_procid: *const c_char,
    _send_stopped: c_int,
) -> TtStatus {
    eprintln!("[libtt_shim] ttdt_close stub called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttdt_open(
    _procid: *mut *const c_char,
    _toolname: *const c_char,
    _vendor: *const c_char,
    _version: *const c_char,
    _send_started: c_int,
) -> *mut c_char {
    // Returns procid
    eprintln!("[libtt_shim] ttdt_open stub called");
    // Return a dummy procid
    CString::new("12345").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn ttdt_session_join(
    _sess: *const c_char,
    _cb: TtCallback,
    _sess_join: *mut c_void, // Tt_message_callback
    _c_data: *mut c_void,
    _join: c_int,
) -> *mut c_void {
    // Returns Tt_pattern *
    eprintln!("[libtt_shim] ttdt_session_join stub called");
    ptr::null_mut()
}

// --- Missing standard TT Stubs ---

#[no_mangle]
pub extern "C" fn tt_message_contexts_count(_m: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn tt_message_context_val(_m: *mut c_void, _c: c_int) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ttmedia_ptype_declare(
    _ptype: *const c_char,
    _base_opnum: c_int,
    _cb: TtCallback,
    _c_data: *mut c_void,
    _declare: c_int,
) -> TtStatus {
    eprintln!("[libtt_shim] ttmedia_ptype_declare stub called");
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_ptype_opnum_callback_add(
    _ptype: *const c_char,
    _opnum: c_int,
    _cb: TtCallback,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tttk_block_while(
    _f: *const c_void, // XtInputMask or similar condition? (const XtInputMask * )
    _t: c_int,
) -> TtStatus {
    // This blocks? We should probably just return for now or loop with sleep?
    // If we return immediately, it might cause busy loops if CDE relies on it for waiting.
    // But implementing a XtMainLoop block here is hard.
    // Assuming minimal usage for now.
    TT_OK
}

// --- Restored and New Stubs ---

#[no_mangle]
pub extern "C" fn tt_message_status_set(_m: *mut c_void, _s: c_int) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_ptype_undeclare(_ptype: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_reject(_m: *mut c_void) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_arg_val_set(
    _m: *mut c_void,
    _n: c_int,
    _val: *const c_char,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_handler_set(_m: *mut c_void, _handler: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_id(_m: *mut c_void) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_X_session(_xdisplay: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_default_session_set(_sess: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttmedia_Deposit(
    _m: *mut c_void,
    _pkey: *const c_char,
    _id: *const c_char,
    _cb: TtCallback,
    _c_data: *mut c_void,
    _contract: *mut c_void,
    _len: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttmedia_load_reply(
    _contract: *mut c_void,
    _new_contents: *const c_uchar,
    _len: c_int,
    _reply_always: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_error_pointer(_e: TtStatus) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_pattern_file_add(_p: *mut c_void, _f: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttmedia_ptype_declare_b(
    _ptype: *const c_char,
    _base_opnum: c_int,
    _cb: TtCallback,
    _c_data: *mut c_void,
    _declare: c_int,
) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn ttdt_file_notice(
    _f: *const c_char,
    _scope: TtScope,
    _join: c_int,
    _cb: TtCallback,
    _c_data: *mut c_void,
) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tt_message_handler_ptype_set(_m: *mut c_void, _ptype: *const c_char) -> TtStatus {
    TT_OK
}

#[no_mangle]
pub extern "C" fn tt_message_context_set(
    _m: *mut c_void,
    _c: *const c_char,
    _val: *const c_char,
) -> TtStatus {
    TT_OK
}
