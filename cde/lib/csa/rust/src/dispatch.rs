use crate::bindings::{bool_t, clnt_stat, CLIENT};
use crate::xdr_c_bindings::{cms_open_args, CSA_return_code};
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

// Constants from csa.h
pub const CSA_SUCCESS: CSA_return_code = 0;
pub const CSA_E_INVALID_PARAMETER: CSA_return_code = 15;
pub const CSA_E_SERVICE_UNAVAILABLE: CSA_return_code = 21;

// Constants from cm.h
pub const TABLEVERS: c_ulong = 5;
pub const TABLEVERS_4: c_ulong = 4;

// Constants from connection.h
pub const _DtCM_LONG_TIMEOUT: c_int = 60;

// Boolean constants
pub const B_TRUE: c_int = 1;
pub const B_FALSE: c_int = 0;

// clnt_stat enum value
pub const RPC_SUCCESS: clnt_stat = 0;

// Forward declarations/opaque types
#[repr(C)]
pub struct _DtCmNameTable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _DtCmCallbackEntry {
    _private: [u8; 0],
}

#[repr(C)]
#[repr(C)]
pub struct cms_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CSA_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CSA_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _DtCm_Target_List {
    pub cal: *mut c_char,
    pub update_type: c_ulong,
    pub next: *mut _DtCm_Target_List,
}

#[repr(C)]
pub struct _DtCm_Client_Info {
    pub host: *mut c_char,
    pub tcpcl: *mut CLIENT,
    pub udpcl: *mut CLIENT,
    pub vers_out: c_ulong,
    pub last_used: c_long,
    pub nregistered: c_int,
    pub tlist: *mut _DtCm_Target_List,
    pub next: *mut _DtCm_Client_Info,
    pub prev: *mut _DtCm_Client_Info,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum _DtCm_Transport_Type {
    tcp_transport = 0,
    udp_transport = 1,
}

#[repr(C)]
pub struct _DtCm_Connection {
    pub ci: *mut _DtCm_Client_Info,
    pub retry: c_int,
    pub use_: _DtCm_Transport_Type,
    pub stat: clnt_stat,
}

#[repr(C)]
pub struct Calendar {
    pub handle: *mut c_void,
    pub rpc_version: c_int,
    pub file_version: c_int,
    pub cal_tbl: *mut _DtCmNameTable,
    pub entry_tbl: *mut _DtCmNameTable,
    pub name: *mut c_char,
    pub location: *mut c_char,
    pub async_process: bool_t,
    pub all_reasons: c_ulong,
    pub do_reasons: c_ulong,
    pub cb_list: *mut _DtCmCallbackEntry,
    pub conn: _DtCm_Connection,
    pub num_attrs: c_uint,
    pub attrs: *mut cms_attribute,
    pub got_attrs: bool_t,
    pub access: c_int,
    pub ehead: *mut c_void,
    pub etail: *mut c_void,
    pub next: *mut Calendar,
    pub prev: *mut Calendar,
}

// External C functions we need to call
extern "C" {
    fn _DtCm_create_tcp_client(
        host: *mut c_char,
        version: c_int,
        timeout: c_int,
        clnt: *mut *mut _DtCm_Client_Info,
    ) -> CSA_return_code;

    fn _DtCm_clntstat_to_csastat(clntstat: clnt_stat) -> CSA_return_code;

    fn _DtCm_rpc_open_calendar_legacy(cal: *mut Calendar) -> CSA_return_code;

    fn getpid() -> c_int;

    // Helper functions from rpccalls.c
    fn csa2cmsattrs(
        num_attrs: c_ulong,
        csaattrs: *mut CSA_attribute,
        ops: *mut CSA_enum,
        newnum: *mut c_ulong,
        newattrs: *mut *mut cms_attribute,
        newops: *mut *mut CSA_enum,
    ) -> CSA_return_code;

    fn free_cmsattrs(num_attrs: c_ulong, attrs: *mut cms_attribute);

    fn _DtCm_rpc_create_calendar_legacy(
        cal: *mut Calendar,
        num_attrs: c_ulong,
        attrs: *mut CSA_attribute,
    ) -> CSA_return_code;
}

/// Rust implementation of the v5 RPC open calendar logic
///
/// This function handles the modern (v5) protocol path in Rust,
/// while falling back to C for legacy versions (v2-v4).
#[no_mangle]
pub unsafe extern "C" fn rs_rpc_open_calendar(cal: *mut Calendar) -> CSA_return_code {
    if cal.is_null() {
        return CSA_E_INVALID_PARAMETER;
    }

    let mut ci: *mut _DtCm_Client_Info = ptr::null_mut();

    // Create TCP client connection
    let stat = _DtCm_create_tcp_client(
        (*cal).location,
        TABLEVERS as c_int,
        _DtCM_LONG_TIMEOUT,
        &mut ci as *mut *mut _DtCm_Client_Info,
    );

    if stat != CSA_SUCCESS {
        return stat;
    }

    // Set connection info
    (*cal).conn.ci = ci;
    (*cal).conn.retry = B_TRUE;

    if (*ci).vers_out == TABLEVERS {
        // v5 protocol - handle in Rust
        let mut args = cms_open_args {
            cal: (*cal).name,
            pid: getpid(),
        };

        // Call the existing Rust implementation
        // Need to use the xdr_c_bindings::_DtCm_Connection type
        let conn_ptr = &mut (*cal).conn as *mut _DtCm_Connection
            as *mut crate::xdr_c_bindings::_DtCm_Connection;
        let res = crate::client::cms_open_calendar_5(&mut args, conn_ptr);

        if !res.is_null() {
            let res_ref = &*res;
            let result_stat = res_ref.stat;

            if result_stat == CSA_SUCCESS {
                (*cal).rpc_version = res_ref.svr_vers as c_int;
                (*cal).file_version = res_ref.file_vers as c_int;
                (*cal).access = res_ref.user_access as c_int;
            }

            // Note: We don't free res here as the Rust client owns it
            // The client should handle cleanup via Drop

            result_stat
        } else {
            // NULL result - check connection status
            if (*cal).conn.stat == RPC_SUCCESS {
                CSA_E_SERVICE_UNAVAILABLE
            } else {
                _DtCm_clntstat_to_csastat((*cal).conn.stat)
            }
        }
    } else {
        // Legacy protocol (v2-v4) - delegate to C implementation
        _DtCm_rpc_open_calendar_legacy(cal)
    }
}

/// Rust implementation of the v5 RPC create calendar logic
#[no_mangle]
pub unsafe extern "C" fn rs_rpc_create_calendar(
    cal: *mut Calendar,
    num_attrs: c_ulong,
    attrs: *mut CSA_attribute,
) -> CSA_return_code {
    if cal.is_null() {
        return CSA_E_INVALID_PARAMETER;
    }

    let mut ci: *mut _DtCm_Client_Info = ptr::null_mut();

    let stat = _DtCm_create_tcp_client(
        (*cal).location,
        TABLEVERS as c_int,
        _DtCM_LONG_TIMEOUT,
        &mut ci as *mut *mut _DtCm_Client_Info,
    );

    if stat != CSA_SUCCESS {
        return stat;
    }

    (*cal).conn.ci = ci;
    (*cal).conn.retry = B_FALSE as c_int;

    if (*ci).vers_out == TABLEVERS {
        let mut num_cmsattrs: c_ulong = 0;
        let mut cmsattrs: *mut cms_attribute = ptr::null_mut();

        let stat = csa2cmsattrs(
            num_attrs,
            attrs,
            ptr::null_mut(),
            &mut num_cmsattrs,
            &mut cmsattrs,
            ptr::null_mut(),
        );

        if stat != CSA_SUCCESS {
            return stat;
        }

        let mut args = crate::xdr_c_bindings::cms_create_args {
            cal: (*cal).name,
            char_set: ptr::null_mut(), // nullstr equivalent? Or need proper pointer. nullstr in C was "".
            pid: getpid(),
            num_attrs: num_cmsattrs as u32,
            attrs: cmsattrs as *mut crate::xdr_c_bindings::cms_attribute,
        };

        // Note: char_set needs to be a valid pointer.
        // In C it was `nullstr = ""`.
        // We can create a static empty C string.
        let empty_str = b"\0";
        args.char_set = empty_str.as_ptr() as *mut c_char;

        let conn_ptr = &mut (*cal).conn as *mut _DtCm_Connection
            as *mut crate::xdr_c_bindings::_DtCm_Connection;

        let res = crate::client::cms_create_calendar_5(&mut args, conn_ptr);

        if num_cmsattrs > 0 {
            free_cmsattrs(num_cmsattrs, cmsattrs);
        }

        if !res.is_null() {
            *res
        } else {
            if (*cal).conn.stat == RPC_SUCCESS {
                CSA_E_SERVICE_UNAVAILABLE
            } else {
                _DtCm_clntstat_to_csastat((*cal).conn.stat)
            }
        }
    } else {
        _DtCm_rpc_create_calendar_legacy(cal, num_attrs, attrs)
    }
}
