use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub enum IpcCommandType {
    Unknown = 0,
    Navigate = 1,
    Shutdown = 2,
}

#[repr(C)]
pub struct IpcCommand {
    pub cmd_type: IpcCommandType,
    pub url: *mut c_char,
}

#[no_mangle]
pub extern "C" fn dtbrowser_ipc_init() {
    eprintln!("[Rust-IPC] Initialized safety layer.");
}

#[no_mangle]
pub extern "C" fn dtbrowser_ipc_parse(op: *const c_char, arg: *const c_char) -> IpcCommand {
    if op.is_null() {
        return IpcCommand {
            cmd_type: IpcCommandType::Unknown,
            url: std::ptr::null_mut(),
        };
    }

    let c_op = unsafe { CStr::from_ptr(op) };
    let op_str = c_op.to_string_lossy();

    let mut cmd_type = IpcCommandType::Unknown;
    let mut url_ptr = std::ptr::null_mut();

    if op_str == "Display_Topic" || op_str == "Open_URL" {
        cmd_type = IpcCommandType::Navigate;
        if !arg.is_null() {
            let c_arg = unsafe { CStr::from_ptr(arg) };
            // In a real implementation, we'd sanitize/validate the URL here
            // For now, just duplicate it back to C
            if let Ok(c_string) = CString::new(c_arg.to_bytes()) {
                url_ptr = c_string.into_raw();
            }
        }
    } else if op_str == "Quit" {
        cmd_type = IpcCommandType::Shutdown;
    }

    IpcCommand {
        cmd_type,
        url: url_ptr,
    }
}

#[no_mangle]
pub extern "C" fn dtbrowser_ipc_free_command(cmd: IpcCommand) {
    if !cmd.url.is_null() {
        unsafe {
            let _ = CString::from_raw(cmd.url);
        }
    }
}
