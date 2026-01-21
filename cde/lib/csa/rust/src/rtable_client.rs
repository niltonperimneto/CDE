use crate::bindings::{bool_t, u_int, xdr_u_int, XDR};
// Note: need to re-export or use correct path for bindings xdr functions.

use crate::client::get_client_handle;
use crate::xdr_c_bindings::{
    Access_Args_4 as Bind_Access_Args_4,
    Access_Status_4 as Bind_Access_Status_4,
    CSA_return_code,
    Registration_Status_4 as Bind_Registration_Status_4,
    // Rename conflicting types to Bind_...
    Table_Res_4 as Bind_Table_Res_4,
    Table_Status_4 as Bind_Table_Status_4,
    _DtCm_Connection,
};
use std::os::raw::c_int;
use std::ptr;

// Opaque types for Cbindgen to see and emit correctly
#[repr(C)]
pub struct Table_Args_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Res_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Status_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Op_Args_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Args_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Status_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_4 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_Status_4 {
    _private: [u8; 1],
}

#[repr(C)]
pub struct Table_Args_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Res_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Args_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Status_3 {
    _private: [u8; 1],
}

#[repr(C)]
pub struct Table_Args_2 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Res_2 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Args_2 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Access_Status_2 {
    _private: [u8; 1],
}

// Missing V3/V2 Opaque Types
#[repr(C)]
pub struct Table_Status_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Table_Status_2 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_Status_3 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_2 {
    _private: [u8; 1],
}
#[repr(C)]
pub struct Registration_Status_2 {
    _private: [u8; 1],
}

// Implement xdr_CSA_return_code shim
#[no_mangle]
pub unsafe extern "C" fn xdr_CSA_return_code(xdrs: *mut XDR, objp: *mut CSA_return_code) -> bool_t {
    xdr_u_int(xdrs, objp as *mut u_int)
}

// ... helper macro ...
macro_rules! xdr_call {
    ($func:expr, $xdrs:expr, $($arg:expr),*) => {
        if $func($xdrs, $($arg),*) == 0 { return 0; }
    };
}

// ... RPC Stubs ...

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_4(
    _arg: *mut Table_Args_4,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    let clnt = get_client_handle(conn);
    if clnt.is_null() {
        return ptr::null_mut();
    }
    static mut RES: Bind_Table_Res_4 = Bind_Table_Res_4 {
        status: crate::xdr_c_bindings::Access_Status_4_access_failed_4,
        res: crate::xdr_c_bindings::Table_Res_List_4 {
            tag: crate::xdr_c_bindings::Table_Res_Type_4_AP_4,
            Table_Res_List_4_u: crate::xdr_c_bindings::Table_Res_List_4__bindgen_ty_1 {
                a: ptr::null_mut(),
            },
        },
    };
    &mut RES as *mut Bind_Table_Res_4 as *mut Table_Res_4
}

// Note: Omitted detailed implementation of change_3/2 for brevity, they should follow same pattern if needed.
// For now shims:

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_create_4(
    _arg: *mut Table_Op_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_4 {
    static mut RES: Bind_Table_Status_4 = crate::xdr_c_bindings::Table_Status_4_ok_4;
    &mut RES as *mut Bind_Table_Status_4 as *mut Table_Status_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    static mut RES: Bind_Table_Res_4 = Bind_Table_Res_4 {
        status: crate::xdr_c_bindings::Access_Status_4_access_failed_4,
        res: crate::xdr_c_bindings::Table_Res_List_4 {
            tag: crate::xdr_c_bindings::Table_Res_Type_4_AP_4,
            Table_Res_List_4_u: crate::xdr_c_bindings::Table_Res_List_4__bindgen_ty_1 {
                a: ptr::null_mut(),
            },
        },
    };
    &mut RES as *mut Bind_Table_Res_4 as *mut Table_Res_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_range_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    static mut RES: Bind_Table_Res_4 = Bind_Table_Res_4 {
        status: crate::xdr_c_bindings::Access_Status_4_access_failed_4,
        res: crate::xdr_c_bindings::Table_Res_List_4 {
            tag: crate::xdr_c_bindings::Table_Res_Type_4_AP_4,
            Table_Res_List_4_u: crate::xdr_c_bindings::Table_Res_List_4__bindgen_ty_1 {
                a: ptr::null_mut(),
            },
        },
    };
    &mut RES as *mut Bind_Table_Res_4 as *mut Table_Res_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_insert_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    static mut RES: Bind_Table_Res_4 = Bind_Table_Res_4 {
        status: crate::xdr_c_bindings::Access_Status_4_access_failed_4,
        res: crate::xdr_c_bindings::Table_Res_List_4 {
            tag: crate::xdr_c_bindings::Table_Res_Type_4_AP_4,
            Table_Res_List_4_u: crate::xdr_c_bindings::Table_Res_List_4__bindgen_ty_1 {
                a: ptr::null_mut(),
            },
        },
    };
    &mut RES as *mut Bind_Table_Res_4 as *mut Table_Res_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_abbreviated_lookup_key_range_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    static mut RES: Bind_Table_Res_4 = Bind_Table_Res_4 {
        status: crate::xdr_c_bindings::Access_Status_4_access_failed_4,
        res: crate::xdr_c_bindings::Table_Res_List_4 {
            tag: crate::xdr_c_bindings::Table_Res_Type_4_AP_4,
            Table_Res_List_4_u: crate::xdr_c_bindings::Table_Res_List_4__bindgen_ty_1 {
                a: ptr::null_mut(),
            },
        },
    };
    &mut RES as *mut Bind_Table_Res_4 as *mut Table_Res_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_set_access_4(
    _arg: *mut Access_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Status_4 {
    static mut RES: Bind_Access_Status_4 = crate::xdr_c_bindings::Access_Status_4_access_failed_4;
    &mut RES as *mut Bind_Access_Status_4 as *mut Access_Status_4
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_get_access_4(
    _arg: *mut Access_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Args_4 {
    static mut RES: Bind_Access_Args_4 = Bind_Access_Args_4 {
        target: ptr::null_mut(),
        access_list: ptr::null_mut(),
    };
    &mut RES as *mut Bind_Access_Args_4 as *mut Access_Args_4
}

// Shims for V2/V3 that just return null for now/noop
// Shims for V2/V3 explicitly typed to match headers
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_range_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_range_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_get_access_3(
    _arg: *mut Access_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Args_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_get_access_2(
    _arg: *mut Access_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Args_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_set_access_3(
    _arg: *mut Access_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Status_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_set_access_2(
    _arg: *mut Access_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Access_Status_2 {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_size_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut c_int {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_size_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut c_int {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_size_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut c_int {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_next_reminder_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_next_reminder_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_lookup_next_reminder_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_check_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_4 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_check_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_check_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_flush_table_4(
    _arg: *mut Table_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_4 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_garbage_collect_4(
    _arg: *mut Table_Op_Args_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Status_4 {
    ptr::null_mut()
}

// Instance Wrappers
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_instance_4(
    arg: *mut Table_Args_4,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    _DtCm_rtable_change_4(arg, conn)
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_instance_3(
    arg: *mut Table_Args_3,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    _DtCm_rtable_change_3(arg, conn)
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_change_instance_2(
    arg: *mut Table_Args_2,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    _DtCm_rtable_change_2(arg, conn)
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_instance_4(
    arg: *mut Table_Args_4,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_4 {
    _DtCm_rtable_delete_4(arg, conn)
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_instance_3(
    arg: *mut Table_Args_3,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    _DtCm_rtable_delete_3(arg, conn)
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_delete_instance_2(
    arg: *mut Table_Args_2,
    conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    _DtCm_rtable_delete_2(arg, conn)
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_insert_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_insert_2(
    _arg: *mut Table_Args_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_2 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_rtable_abbreviated_lookup_key_range_3(
    _arg: *mut Table_Args_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Table_Res_3 {
    ptr::null_mut()
}

// XDR routines are now provided by xdr_stubs.rs via xdrgen

// Registration Callbacks
#[no_mangle]
pub unsafe extern "C" fn _DtCm_register_callback_4(
    _arg: *mut Registration_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_4 {
    static mut RES: Bind_Registration_Status_4 =
        crate::xdr_c_bindings::Registration_Status_4_failed_4;
    &mut RES as *mut Bind_Registration_Status_4 as *mut Registration_Status_4
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_register_callback_3(
    _arg: *mut Registration_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_register_callback_2(
    _arg: *mut Registration_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_2 {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn _DtCm_deregister_callback_4(
    _arg: *mut Registration_4,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_4 {
    static mut RES: Bind_Registration_Status_4 =
        crate::xdr_c_bindings::Registration_Status_4_failed_4;
    &mut RES as *mut Bind_Registration_Status_4 as *mut Registration_Status_4
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_deregister_callback_3(
    _arg: *mut Registration_3,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_3 {
    ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _DtCm_deregister_callback_2(
    _arg: *mut Registration_2,
    _conn: *mut _DtCm_Connection,
) -> *mut Registration_Status_2 {
    ptr::null_mut()
}
