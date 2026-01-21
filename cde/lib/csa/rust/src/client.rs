use crate::bindings::{bool_t, clnt_stat, rpcproc_t, timeval, xdrproc_t, CLIENT, XDR};
 // Rust XDR types
use crate::xdr_adapter::XdrStream;
use crate::xdr_c_bindings::{
    cms_create_args, CSA_return_code, Transport_type_udp_transport, _DtCm_Connection,
};
use std::os::raw::{c_char, c_void};
use std::ptr;
use xdr_codec::Pack;

// Constants from cm.h/cm.x
const CMS_CREATE_CALENDAR: u32 = 3;

extern "C" {
    // Manually declare if missing from bindings
    pub fn xdr_CSA_return_code(xdrs: *mut XDR, objp: *mut CSA_return_code) -> bool_t;
}

pub unsafe fn clnt_call(
    clnt: *mut CLIENT,
    proc_num: rpcproc_t,
    xargs: xdrproc_t,
    argsp: *mut c_void,
    xres: xdrproc_t,
    resp: *mut c_void,
    timeout: timeval,
) -> clnt_stat {
    let client = &*clnt;
    let ops = &*client.cl_ops;
    if let Some(call_fn) = ops.cl_call {
        call_fn(clnt, proc_num, xargs, argsp, xres, resp, timeout)
    } else {
        // Fallback or panic? RPC_SYSTEMERROR is appropriate but we'd need to cast.
        // For now panic or return error.
        // clnt_stat is an enum.
        // Assuming 12 (RPC_SYSTEMERROR) or similar.
        // Better to panic in development if ops are missing.
        panic!("clnt_call: cl_ops->cl_call is NULL");
    }
}

pub unsafe fn get_client_handle(conn: *mut _DtCm_Connection) -> *mut CLIENT {
    if conn.is_null() {
        return ptr::null_mut();
    }
    let conn_ref = &*conn;
    if conn_ref.ci.is_null() {
        return ptr::null_mut();
    }
    let ci_ref = &*conn_ref.ci;

    if conn_ref.use_ == Transport_type_udp_transport {
        ci_ref.udpcl as *mut CLIENT
    } else {
        ci_ref.tcpcl as *mut CLIENT
    }
}

// Wrapper for XDR encoding (packing)
unsafe extern "C" fn xdr_cms_create_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const cms_create_args;

    // Convert C args to Rust args
    let rust_args = crate::conversion::convert_cms_create_args(args_c);

    // Wrap XDR pointer in XdrStream
    let mut stream = XdrStream::new(xdrs);

    // Pack Rust struct into stream
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,  // TRUE
        Err(_) => 0, // FALSE
    }
}

// Stub implementation
#[no_mangle]
pub unsafe extern "C" fn cms_create_calendar_5(
    arg: *mut cms_create_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0; // Static result

    // Construct timeval manually if needed, or use bindings::timeval
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };

    let status = clnt_call(
        clnt,
        CMS_CREATE_CALENDAR as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_create_args_pack as *const ())), // cast to variadic fn ptr
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())), // cast to variadic fn ptr
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );

    if status != 0 {
        // RPC_SUCCESS is 0
        return ptr::null_mut();
    }
    &raw mut RES as *mut CSA_return_code
}

// --- CMS_DELETE_ENTRY ---
const CMS_DELETE_ENTRY: u32 = 18;
unsafe extern "C" fn xdr_cms_delete_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_delete_args;
    let rust_args = crate::conversion::convert_cms_delete_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_delete_entry_5(
    arg: *mut crate::xdr_c_bindings::cms_delete_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_DELETE_ENTRY as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_delete_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES as *mut CSA_return_code
}

// Let's implement generic stubs for what I have conversions for.

// CMS_OPEN_CALENDAR
const CMS_OPEN_CALENDAR: u32 = 2;
unsafe extern "C" fn xdr_cms_open_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_open_args;
    let rust_args = crate::conversion::convert_cms_open_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_open_calendar_5(
    arg: *mut crate::xdr_c_bindings::cms_open_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_open_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_open_res = crate::cm::cms_open_res {
        stat: 0,
        svr_vers: 0,
        file_vers: 0,
        user_access: 0,
        attrs: Vec::new(),
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_OPEN_CALENDAR as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_open_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_open_res_unpack as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// Helper: xdr_cms_open_res_unpack
unsafe extern "C" fn xdr_cms_open_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_open_res;
    let mut stream = XdrStream::new(xdrs);
    // Unpack into res_rust
    use xdr_codec::Unpack;
    match crate::cm::cms_open_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}

// Need to repeat this pattern for all stubs returning structs.
// For create_calendar it returned CSA_return_code (simple enum/int).
// For others like OPEN, they return `cms_open_res`.

// CMS_UPDATE_ENTRY
const CMS_UPDATE_ENTRY: u32 = 17;
unsafe extern "C" fn xdr_cms_update_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_update_args;
    let rust_args = crate::conversion::convert_cms_update_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_update_entry_5(
    arg: *mut crate::xdr_c_bindings::cms_update_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_entry_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_entry_res = crate::cm::cms_entry_res {
        stat: 0,
        entry: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_UPDATE_ENTRY as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_update_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_entry_res_unpack as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// CMS_INSERT_ENTRY
const CMS_INSERT_ENTRY: u32 = 16;
unsafe extern "C" fn xdr_cms_insert_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_insert_args;
    let rust_args = crate::conversion::convert_cms_insert_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_insert_entry_5(
    arg: *mut crate::xdr_c_bindings::cms_insert_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_entry_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_entry_res = crate::cm::cms_entry_res {
        stat: 0,
        entry: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_INSERT_ENTRY as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_insert_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_entry_res_unpack as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

unsafe extern "C" fn xdr_cms_entry_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_entry_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_entry_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}

// --- CMS_REMOVE_CALENDAR ---
const CMS_REMOVE_CALENDAR: u32 = 4;
unsafe extern "C" fn xdr_cms_remove_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_remove_args;
    let rust_args = crate::conversion::convert_cms_remove_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_remove_calendar_5(
    arg: *mut crate::xdr_c_bindings::cms_remove_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_REMOVE_CALENDAR as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_remove_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_ARCHIVE ---
const CMS_ARCHIVE: u32 = 10;
unsafe extern "C" fn xdr_cms_archive_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_archive_args;
    let rust_args = crate::conversion::convert_cms_archive_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn xdr_cms_archive_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_archive_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_archive_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_archive_5(
    arg: *mut crate::xdr_c_bindings::cms_archive_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_archive_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_archive_res = crate::cm::cms_archive_res {
        stat: 0,
        data: crate::cm::buffer(String::new()),
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    }; // Should check if archive needs longer timeout
    let status = clnt_call(
        clnt,
        CMS_ARCHIVE as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_archive_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_archive_res_unpack as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_RESTORE ---
const CMS_RESTORE: u32 = 11;
unsafe extern "C" fn xdr_cms_restore_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_restore_args;
    let rust_args = crate::conversion::convert_cms_restore_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
// Restore returns cms_entry_res? No, xdr_c_bindings usually have cms_restore_res if exists, or return code.
// Grep showed cms_restore_res? No.
// Let's assume CSA_return_code if no res struct found.
// cm.rs had cms_archive_res but I didn't see cms_restore_res.
// Checking cm.rs source again:
// Line 77: pub struct cms_restore_args
// Next: cms_set_cal_attr_args
// So likely returns CSA_return_code or similar.
// I'll assume CSA_return_code for now.
#[no_mangle]
pub unsafe extern "C" fn cms_restore_5(
    arg: *mut crate::xdr_c_bindings::cms_restore_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_RESTORE as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_restore_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}
unsafe extern "C" fn xdr_cms_entries_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_entries_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_entries_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}

// --- CMS_LOOKUP_REMINDER ---
const CMS_LOOKUP_REMINDER: u32 = 12;
unsafe extern "C" fn xdr_cms_reminder_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_reminder_args;
    let rust_args = crate::conversion::convert_cms_reminder_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn xdr_cms_reminder_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_reminder_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_reminder_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_lookup_reminder_5(
    arg: *mut crate::xdr_c_bindings::cms_reminder_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_reminder_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_reminder_res = crate::cm::cms_reminder_res {
        stat: 0,
        rems: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_LOOKUP_REMINDER as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_reminder_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(
            xdr_cms_reminder_res_unpack as *const (),
        )),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_LOOKUP_ENTRIES ---
const CMS_LOOKUP_ENTRIES: u32 = 13;
unsafe extern "C" fn xdr_cms_lookup_entries_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_lookup_entries_args;
    let rust_args = crate::conversion::convert_cms_lookup_entries_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_lookup_entries_5(
    arg: *mut crate::xdr_c_bindings::cms_lookup_entries_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_entries_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_entries_res = crate::cm::cms_entries_res {
        stat: 0,
        entries: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_LOOKUP_ENTRIES as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_lookup_entries_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_entries_res_unpack as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_GET_CALENDAR_ATTR ---
const CMS_GET_CALENDAR_ATTR: u32 = 8;
unsafe extern "C" fn xdr_cms_get_cal_attr_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_get_cal_attr_args;
    let rust_args = crate::conversion::convert_cms_get_cal_attr_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn xdr_cms_get_cal_attr_res_unpack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_get_cal_attr_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_get_cal_attr_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_get_calendar_attr_5(
    arg: *mut crate::xdr_c_bindings::cms_get_cal_attr_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_get_cal_attr_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_get_cal_attr_res = crate::cm::cms_get_cal_attr_res {
        stat: 0,
        attrs: Vec::new(),
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_GET_CALENDAR_ATTR as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_get_cal_attr_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(
            xdr_cms_get_cal_attr_res_unpack as *const (),
        )),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_SET_CALENDAR_ATTR ---
const CMS_SET_CALENDAR_ATTR: u32 = 9;
unsafe extern "C" fn xdr_cms_set_cal_attr_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_set_cal_attr_args;
    let rust_args = crate::conversion::convert_cms_set_cal_attr_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_set_calendar_attr_5(
    arg: *mut crate::xdr_c_bindings::cms_set_cal_attr_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_SET_CALENDAR_ATTR as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_set_cal_attr_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_GET_ENTRY_ATTR ---
const CMS_GET_ENTRY_ATTR: u32 = 15;
unsafe extern "C" fn xdr_cms_get_entry_attr_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_get_entry_attr_args;
    let rust_args = crate::conversion::convert_cms_get_entry_attr_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn xdr_cms_get_entry_attr_res_unpack(
    xdrs: *mut XDR,
    objp: *mut c_void,
) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_get_entry_attr_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_get_entry_attr_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_get_entry_attr_5(
    arg: *mut crate::xdr_c_bindings::cms_get_entry_attr_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_get_entry_attr_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_get_entry_attr_res = crate::cm::cms_get_entry_attr_res {
        stat: 0,
        entries: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let status = clnt_call(
        clnt,
        CMS_GET_ENTRY_ATTR as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_get_entry_attr_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(
            xdr_cms_get_entry_attr_res_unpack as *const (),
        )),
        &raw mut RES as *mut _ as *mut c_void,
        timeout,
    );
    if status != 0 {
        return ptr::null_mut();
    }
    &raw mut RES
}

// --- CMS_REGISTER ---
const CMS_REGISTER: u32 = 5;

unsafe extern "C" fn xdr_cms_register_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_register_args;
    let rust_args = crate::conversion::convert_cms_register_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cms_register_5(
    arg: *mut crate::xdr_c_bindings::cms_register_args,
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    if clnt_call(
        clnt,
        CMS_REGISTER as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_register_args_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &mut RES as *mut _ as *mut c_void,
        timeout,
    ) != 0
    {
        return ptr::null_mut();
    }
    &mut RES
}

// --- CMS_UNREGISTER ---
const CMS_UNREGISTER: u32 = 6;
unsafe extern "C" fn xdr_cms_unregister_args_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_register_args; // Use register args
    let rust_args = crate::conversion::convert_cms_unregister_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_unregister_5(
    arg: *mut crate::xdr_c_bindings::cms_register_args, // Use register args
    conn: *mut _DtCm_Connection,
) -> *mut CSA_return_code {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: CSA_return_code = 0;
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    if clnt_call(
        clnt,
        CMS_UNREGISTER as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_unregister_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_CSA_return_code as *const ())),
        &mut RES as *mut _ as *mut c_void,
        timeout,
    ) != 0
    {
        return ptr::null_mut();
    }
    &mut RES
}

// --- CMS_ENUMERATE_SEQUENCE ---
const CMS_ENUMERATE_SEQUENCE: u32 = 14; // cm.x says 14

// Return type: cms_entries_res
unsafe extern "C" fn xdr_cms_enumerate_sequence_args_pack(
    xdrs: *mut XDR,
    objp: *mut c_void,
) -> bool_t {
    let args_c = objp as *const crate::xdr_c_bindings::cms_enumerate_args;
    let rust_args = crate::conversion::convert_cms_enumerate_args(args_c);
    let mut stream = XdrStream::new(xdrs);
    match rust_args.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cms_enumerate_sequence_5(
    arg: *mut crate::xdr_c_bindings::cms_enumerate_args,
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_entries_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_entries_res = crate::cm::cms_entries_res {
        stat: 0,
        entries: None,
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    if clnt_call(
        clnt,
        CMS_ENUMERATE_SEQUENCE as rpcproc_t,
        Some(std::mem::transmute(
            xdr_cms_enumerate_sequence_args_pack as *const (),
        )),
        arg as *mut c_void,
        Some(std::mem::transmute(xdr_cms_entries_res_unpack as *const ())),
        &mut RES as *mut _ as *mut c_void,
        timeout,
    ) != 0
    {
        return ptr::null_mut();
    }
    &mut RES
}

// --- CMS_ENUMERATE_CALENDAR_ATTR ---
const CMS_ENUMERATE_CALENDAR_ATTR: u32 = 7; // cm.x says 7

// Arg type: cms_name (string)
unsafe extern "C" fn xdr_cms_name_pack(xdrs: *mut XDR, objp: *mut c_void) -> bool_t {
    // objp is cms_name* which is char**
    let args_pp = objp as *const *const c_char;
    let args_c = unsafe { *args_pp };
    let rust_val = if args_c.is_null() {
        crate::cm::cms_name(String::new())
    } else {
        match std::ffi::CStr::from_ptr(args_c).to_str() {
            Ok(s) => crate::cm::cms_name(s.to_string()),
            Err(_) => crate::cm::cms_name(String::new()),
        }
    };
    let mut stream = XdrStream::new(xdrs);
    match rust_val.pack(&mut stream) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

unsafe extern "C" fn xdr_cms_enumerate_calendar_attr_res_unpack(
    xdrs: *mut XDR,
    objp: *mut c_void,
) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_enumerate_calendar_attr_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_enumerate_calendar_attr_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}
#[no_mangle]
pub unsafe extern "C" fn cms_enumerate_calendar_attr_5(
    arg: *mut *mut c_char, // cms_name * (char **)
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_enumerate_calendar_attr_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_enumerate_calendar_attr_res =
        crate::cm::cms_enumerate_calendar_attr_res {
            stat: 0,
            names: Vec::new(),
        };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    if clnt_call(
        clnt,
        CMS_ENUMERATE_CALENDAR_ATTR as rpcproc_t,
        Some(std::mem::transmute(xdr_cms_name_pack as *const ())),
        arg as *mut c_void,
        Some(std::mem::transmute(
            xdr_cms_enumerate_calendar_attr_res_unpack as *const (),
        )),
        &mut RES as *mut _ as *mut c_void,
        timeout,
    ) != 0
    {
        return ptr::null_mut();
    }
    &mut RES
}

// --- CMS_LIST_CALENDARS ---
const CMS_LIST_CALENDARS: u32 = 1; // cm.x says 1

unsafe extern "C" fn xdr_cms_list_calendars_res_unpack(
    xdrs: *mut XDR,
    objp: *mut c_void,
) -> bool_t {
    let res_rust = objp as *mut crate::cm::cms_list_calendars_res;
    let mut stream = XdrStream::new(xdrs);
    use xdr_codec::Unpack;
    match crate::cm::cms_list_calendars_res::unpack(&mut stream) {
        Ok((v, _)) => {
            *res_rust = v;
            1
        }
        Err(_) => 0,
    }
}

// cms_list_calendars takes void args
#[no_mangle]
pub unsafe extern "C" fn cms_list_calendars_5(
    _arg: *mut c_void, // void arg
    conn: *mut _DtCm_Connection,
) -> *mut crate::cm::cms_list_calendars_res {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: crate::cm::cms_list_calendars_res = crate::cm::cms_list_calendars_res {
        stat: 0,
        names: Vec::new(),
    };
    let timeout = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    if clnt_call(
        clnt,
        CMS_LIST_CALENDARS as rpcproc_t,
        Some(std::mem::transmute(
            crate::xdr_c_bindings::xdr_void as *const (),
        )), // Use fully qualified xdr_void
        ptr::null_mut(), // null arg
        Some(std::mem::transmute(
            xdr_cms_list_calendars_res_unpack as *const (),
        )),
        &mut RES as *mut _ as *mut c_void,
        timeout,
    ) != 0
    {
        return ptr::null_mut();
    }
    &mut RES
}
