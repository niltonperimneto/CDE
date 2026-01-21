use crate::cm as r; // Rust XDR types
use crate::xdr_c_bindings as c;
use std::ffi::CStr;
use std::slice;

// Helper to convert C strings
unsafe fn convert_string(ptr: *mut std::os::raw::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

// Helper to convert buffer
unsafe fn convert_buffer(buf: c::buffer) -> r::buffer {
    r::buffer(convert_string(buf))
}

// Convert cms_attr_name
unsafe fn convert_attr_name(c_name: c::cms_attr_name) -> r::cms_attr_name {
    r::cms_attr_name {
        num: c_name.num as i32,
        name: r::cms_name(convert_string(c_name.name)),
    }
}

// Convert cms_access_entry linked list
unsafe fn convert_access_list(ptr: *mut c::cms_access_entry) -> Option<Box<r::cms_access_entry>> {
    if ptr.is_null() {
        return None;
    }
    let node = *ptr;
    Some(Box::new(r::cms_access_entry {
        user: r::cms_name(convert_string(node.user)),
        rights: node.rights,
        next: convert_access_list(node.next),
    }))
}

// Convert cms_attribute_value
unsafe fn convert_attribute_value(
    c_val_ptr: *mut c::cms_attribute_value,
) -> Option<Box<r::cms_attribute_value>> {
    if c_val_ptr.is_null() {
        return None;
    }
    let c_val = *c_val_ptr;
    let type_ = c_val.type_;
    let item = c_val.item;

    let val = match type_ {
        0 => r::cms_attribute_value::Const0(item.boolean_value as u32),
        1 => r::cms_attribute_value::Const1(item.enumerated_value as i32),
        2 => r::cms_attribute_value::Const2(item.flags_value as u32),
        3 => r::cms_attribute_value::Const3(item.sint32_value as i32),
        4 => r::cms_attribute_value::Const4(item.uint32_value as u32),
        5 => r::cms_attribute_value::Const5(convert_string(item.string_value)),
        6 => r::cms_attribute_value::Const6(convert_string(item.calendar_user_value)),
        7 => r::cms_attribute_value::Const7(convert_string(item.date_time_value)),
        8 => r::cms_attribute_value::Const8(convert_string(item.date_time_range_value)),
        9 => r::cms_attribute_value::Const9(convert_string(item.time_duration_value)),
        10 => r::cms_attribute_value::Const10(convert_access_list(item.access_list_value)),
        // 11 missing
        12 => {
            // CSA_date_time_entry linked list
            // TODO: Implement recursive conversion for date_time_list if needed
            r::cms_attribute_value::Const12(None)
        }
        13 => r::cms_attribute_value::Const13(None), // Reminder
        14 => r::cms_attribute_value::Const14(None), // Opaque
        _ => r::cms_attribute_value::default,
    };

    Some(Box::new(val))
}

// Convert cms_attribute
unsafe fn convert_attribute(c_attr: c::cms_attribute) -> r::cms_attribute {
    r::cms_attribute {
        name: convert_attr_name(c_attr.name),
        value: convert_attribute_value(c_attr.value),
    }
}

// Main conversion for cms_create_args
pub unsafe fn convert_cms_create_args(args: *const c::cms_create_args) -> r::cms_create_args {
    let args = *args;

    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };

    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    r::cms_create_args {
        cal: r::cms_name(convert_string(args.cal)),
        char_set: convert_buffer(args.char_set),
        pid: args.pid,
        attrs: attrs_vec,
    }
}

// Convert cms_key
unsafe fn convert_key(k: c::cms_key) -> r::cms_key {
    r::cms_key {
        time: k.time as i64,
        id: k.id as i32,
    }
}

// Convert cms_delete_args
pub unsafe fn convert_cms_delete_args(args: *const c::cms_delete_args) -> r::cms_delete_args {
    let args = *args;
    r::cms_delete_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
        entry: convert_key(args.entry),
        scope: args.scope,
    }
}

// Convert cms_open_args
pub unsafe fn convert_cms_open_args(args: *const c::cms_open_args) -> r::cms_open_args {
    let args = *args;
    r::cms_open_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
    }
}

// Convert cms_remove_args
pub unsafe fn convert_cms_remove_args(args: *const c::cms_remove_args) -> r::cms_remove_args {
    let args = *args;
    r::cms_remove_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
    }
}

// Convert cms_update_args
pub unsafe fn convert_cms_update_args(args: *const c::cms_update_args) -> r::cms_update_args {
    let args = *args;

    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };

    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    r::cms_update_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
        entry: convert_key(args.entry),
        scope: args.scope,
        attrs: attrs_vec,
    }
}

// Convert cms_insert_args
pub unsafe fn convert_cms_insert_args(args: *const c::cms_insert_args) -> r::cms_insert_args {
    let args = *args;

    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };

    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    r::cms_insert_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
        attrs: attrs_vec,
    }
}

// Convert cms_archive_args
pub unsafe fn convert_cms_archive_args(args: *const c::cms_archive_args) -> r::cms_archive_args {
    let args = *args;
    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };
    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    let ops = if args.ops.is_null() {
        None
    } else {
        Some(*args.ops as i32)
    };

    r::cms_archive_args {
        cal: r::cms_name(convert_string(args.cal)),
        delete: args.delete != 0,
        char_set: convert_buffer(args.char_set),
        attrs: attrs_vec,
        ops: ops,
    }
}

// Convert cms_restore_args
pub unsafe fn convert_cms_restore_args(args: *const c::cms_restore_args) -> r::cms_restore_args {
    let args = *args;
    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };
    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    let ops = if args.ops.is_null() {
        None
    } else {
        Some(*args.ops as i32)
    };

    r::cms_restore_args {
        cal: r::cms_name(convert_string(args.cal)),
        data: convert_buffer(args.data),
        char_set: convert_buffer(args.char_set),
        attrs: attrs_vec,
        ops: ops,
    }
}

// Convert cms_reminder_args
pub unsafe fn convert_cms_reminder_args(args: *const c::cms_reminder_args) -> r::cms_reminder_args {
    let args = *args;
    let names_slice = if args.names.is_null() || args.num_names == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.names, args.num_names as usize)
    };
    let names_vec: Vec<r::cms_attr_name> =
        names_slice.iter().map(|&n| convert_attr_name(n)).collect();

    r::cms_reminder_args {
        cal: r::cms_name(convert_string(args.cal)),
        tick: args.tick as i64,
        names: names_vec,
    }
}

// Convert cms_lookup_entries_args
pub unsafe fn convert_cms_lookup_entries_args(
    args: *const c::cms_lookup_entries_args,
) -> r::cms_lookup_entries_args {
    let args = *args;
    let attr = if args.attrs.is_null() || args.num_attrs == 0 {
        None
    } else {
        Some(Box::new(convert_attribute(*args.attrs)))
    };

    let ops = if args.ops.is_null() {
        None
    } else {
        Some(*args.ops as i32)
    };

    r::cms_lookup_entries_args {
        cal: r::cms_name(convert_string(args.cal)),
        char_set: convert_buffer(args.char_set),
        num_attrs: args.num_attrs,
        attrs: attr,
        ops: ops,
    }
}

// Convert cms_get_cal_attr_args
pub unsafe fn convert_cms_get_cal_attr_args(
    args: *const c::cms_get_cal_attr_args,
) -> r::cms_get_cal_attr_args {
    let args = *args;
    let names_slice = if args.names.is_null() || args.num_names == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.names, args.num_names as usize)
    };
    let names_vec: Vec<r::cms_attr_name> =
        names_slice.iter().map(|&n| convert_attr_name(n)).collect();

    r::cms_get_cal_attr_args {
        cal: r::cms_name(convert_string(args.cal)),
        names: names_vec,
    }
}

// Convert cms_set_cal_attr_args
pub unsafe fn convert_cms_set_cal_attr_args(
    args: *const c::cms_set_cal_attr_args,
) -> r::cms_set_cal_attr_args {
    let args = *args;
    let attrs_slice = if args.attrs.is_null() || args.num_attrs == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.attrs, args.num_attrs as usize)
    };
    let attrs_vec: Vec<r::cms_attribute> =
        attrs_slice.iter().map(|&a| convert_attribute(a)).collect();

    r::cms_set_cal_attr_args {
        cal: r::cms_name(convert_string(args.cal)),
        pid: args.pid,
        attrs: attrs_vec,
    }
}

// Convert cms_get_entry_attr_args
pub unsafe fn convert_cms_get_entry_attr_args(
    args: *const c::cms_get_entry_attr_args,
) -> r::cms_get_entry_attr_args {
    let args = *args;
    let keys_slice = if args.keys.is_null() || args.num_keys == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.keys, args.num_keys as usize)
    };
    let keys_vec: Vec<r::cms_key> = keys_slice.iter().map(|&k| convert_key(k)).collect();

    let names_slice = if args.names.is_null() || args.num_names == 0 {
        &[]
    } else {
        slice::from_raw_parts(args.names, args.num_names as usize)
    };
    let names_vec: Vec<r::cms_attr_name> =
        names_slice.iter().map(|&n| convert_attr_name(n)).collect();

    r::cms_get_entry_attr_args {
        cal: r::cms_name(convert_string(args.cal)),
        keys: keys_vec,
        names: names_vec,
    }
}

// Convert cms_register_args
pub unsafe fn convert_cms_register_args(args: *const c::cms_register_args) -> r::cms_register_args {
    let args = *args;
    r::cms_register_args {
        cal: r::cms_name(convert_string(args.cal)),
        update_type: args.update_type,
        prognum: args.prognum as u64, // casting u_long to u64
        versnum: args.versnum as u64,
        procnum: args.procnum as u64,
        pid: args.pid,
    }
}

// Convert cms_unregister_args (uses register args)
pub unsafe fn convert_cms_unregister_args(
    args: *const c::cms_register_args,
) -> r::cms_register_args {
    convert_cms_register_args(args)
}

// Convert cms_enumerate_args
pub unsafe fn convert_cms_enumerate_args(
    args: *const c::cms_enumerate_args,
) -> r::cms_enumerate_args {
    let args = *args;
    r::cms_enumerate_args {
        cal: r::cms_name(convert_string(args.cal)),
        id: args.id as i32,
        start: args.start as i32,
        end: args.end as i32,
    }
}
