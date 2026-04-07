use crate::bindings::{self, XDR};
use crate::xdr_adapter::XdrStream;
use xdr_codec;

macro_rules! impl_xdr_func {
    ($func_name:ident, $type_path:path) => {
        #[no_mangle]
        pub extern "C" fn $func_name(xdrs: *mut XDR, obj: *mut $type_path) -> i32 {
            let op = unsafe { (*xdrs).x_op };
            let mut stream = unsafe { XdrStream::new(xdrs) };

            match op {
                bindings::xdr_op_XDR_ENCODE => {
                    let val = unsafe { &*obj };
                    match xdr_codec::pack(val, &mut stream) {
                        Ok(_) => 1,  // TRUE
                        Err(_) => 0, // FALSE
                    }
                }
                bindings::xdr_op_XDR_DECODE => match xdr_codec::unpack(&mut stream) {
                    Ok(v) => {
                        unsafe {
                            *obj = v;
                        }
                        1
                    }
                    Err(_) => 0,
                },
                bindings::xdr_op_XDR_FREE => {
                    // XDR_FREE means "release any heap data that the XDR decode
                    // step allocated inside *obj".  The surrounding struct itself
                    // belongs to the C caller and must NOT be freed here.
                    //
                    // `drop_in_place` would run the Rust destructor and then
                    // leave the object in a destroyed (invalid) state while the
                    // C caller still holds a live pointer — use-after-free / UB.
                    //
                    // The correct approach is to overwrite *obj with a freshly
                    // default-initialised value.  This drops the old Rust-owned
                    // heap data (Vecs, Strings, Boxes) via the normal Drop glue
                    // while leaving the allocation pointed to by `obj` intact and
                    // in a valid, zeroed state for the C runtime.
                    unsafe {
                        std::ptr::write(obj, std::mem::zeroed());
                    }
                    1
                }
                _ => 0,
            }
        }
    };
}

// Implement stubs for rtable4
use crate::rtable4;
impl_xdr_func!(_DtCm_xdr_Table_Res_4, rtable4::Table_Res_4);
impl_xdr_func!(_DtCm_xdr_Table_Args_4, rtable4::Table_Args_4);
impl_xdr_func!(_DtCm_xdr_Table_Status_4, rtable4::Table_Status_4);
impl_xdr_func!(_DtCm_xdr_Access_Status_4, rtable4::Access_Status_4);
impl_xdr_func!(_DtCm_xdr_Access_Args_4, rtable4::Access_Args_4);
impl_xdr_func!(_DtCm_xdr_Table_Res_Type_4, rtable4::Table_Res_Type_4);
impl_xdr_func!(_DtCm_xdr_Id_4, rtable4::Id_4);
impl_xdr_func!(_DtCm_xdr_Uid_4, rtable4::Uid_4);
impl_xdr_func!(_DtCm_xdr_Tag_4, rtable4::Tag_4);
impl_xdr_func!(_DtCm_xdr_Period_4, rtable4::Period_4);
impl_xdr_func!(_DtCm_xdr_Attribute_4, rtable4::Attribute_4);
impl_xdr_func!(_DtCm_xdr_Except_4, rtable4::Except_4);
impl_xdr_func!(_DtCm_xdr_Appt_4, rtable4::Appt_4);
impl_xdr_func!(_DtCm_xdr_Abb_Appt_4, rtable4::Abb_Appt_4);
impl_xdr_func!(_DtCm_xdr_Reminder_4, rtable4::Reminder_4);
impl_xdr_func!(_DtCm_xdr_Table_Res_List_4, rtable4::Table_Res_List_4);

// Implement stubs for rtable3
use crate::rtable3;
impl_xdr_func!(_DtCm_xdr_Table_Res_3, rtable3::Table_Res_3);
impl_xdr_func!(_DtCm_xdr_Table_Args_3, rtable3::Table_Args_3);
impl_xdr_func!(_DtCm_xdr_Table_Status_3, rtable3::Table_Status_3);
impl_xdr_func!(_DtCm_xdr_Access_Status_3, rtable3::Access_Status_3);
impl_xdr_func!(_DtCm_xdr_Access_Args_3, rtable3::Access_Args_3);

// Implement stubs for rtable2
use crate::rtable2;
impl_xdr_func!(_DtCm_xdr_Table_Res_2, rtable2::Table_Res_2);
impl_xdr_func!(_DtCm_xdr_Table_Args_2, rtable2::Table_Args_2);
impl_xdr_func!(_DtCm_xdr_Table_Status_2, rtable2::Table_Status_2);
impl_xdr_func!(_DtCm_xdr_Access_Status_2, rtable2::Access_Status_2);
impl_xdr_func!(_DtCm_xdr_Access_Args_2, rtable2::Access_Args_2);
