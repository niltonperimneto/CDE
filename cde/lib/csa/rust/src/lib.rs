// FFI shim crate: every public function is `unsafe extern "C"` and operates
// on raw C pointers.  Requiring nested `unsafe {}` blocks inside unsafe fn
// bodies adds verbosity without meaningful safety benefit here.
#![allow(unsafe_op_in_unsafe_fn)]

// Alias cde_xdr as xdr_codec so xdrgen-generated code and existing imports work unchanged.
extern crate cde_xdr as xdr_codec;

// Include generated modules
pub mod agent {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/agent.rs"));
}
pub mod rtable4 {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]
    use crate::cm::*; // Import common types
    type time_t = i64;
    type u_long = u64; // Linux long is 64-bit
    type u_int = u32;
    include!(concat!(env!("OUT_DIR"), "/rtable4.rs"));
}
mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/tirpc_bindings.rs"));
}

mod xdr_c_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/xdr_c_bindings.rs"));
}
pub mod client;
pub mod conversion;
pub mod dispatch;
pub mod rtable_client;
pub mod xdr_adapter;
pub mod xdr_stubs;

pub mod rtable3 {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]
    use crate::cm::*; // Import common types
    type time_t = i64;
    type u_long = u64;
    type u_int = u32;
    include!(concat!(env!("OUT_DIR"), "/rtable3.rs"));
}
pub mod rtable2 {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]
    use crate::cm::*; // Import common types
    type time_t = i64;
    type u_long = u64;
    type u_int = u32;
    include!(concat!(env!("OUT_DIR"), "/rtable2.rs"));
}
pub mod cm {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]
    pub type CSA_return_code = u64; // c_ulong on 64-bit Linux
                                    // other CSA types are generated in cm.rs

    // Common types
    type time_t = i64;
    type u_long = u64;
    type u_int = u32;
    type uint = u32; // Just in case

    // Definitions provided by xdrgen (via build.rs injection)
    // - cms_attribute_value
    // - CSA_reminder
    // - CSA_date_time_entry
    // - CSA_opaque_data

    include!(concat!(env!("OUT_DIR"), "/cm.rs"));
}
// cmcb.x might be needed too, check if it exists and builds

use std::ffi::c_void;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Update_Status {
    update_succeeded = 0,
    update_failed = 1,
}


#[allow(unsafe_attributes, unused_attributes)]
#[no_mangle]
pub extern "C" fn xdr_Update_Status(_xdrs: *mut c_void, _obj: *mut Update_Status) -> i32 {
    1 // TRUE
}
