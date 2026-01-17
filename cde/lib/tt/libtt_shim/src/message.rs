use crate::{TtClass, TtMode, TtScope};
use libc::{c_char, c_int};
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub enum TtArg {
    Int(i32),
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct TtMessage {
    pub op: String,
    pub scope: TtScope,
    pub class: TtClass,
    pub args: Vec<TtArg>,
    pub file: Option<String>,
    pub session: Option<String>,
    pub sender: Option<String>,
    pub handler: Option<String>,
    pub address: c_int, // Tt_address
}

impl TtMessage {
    pub fn new() -> Self {
        Self {
            op: String::new(),
            scope: 0,
            class: 0,
            args: Vec::new(),
            file: None,
            session: None,
            sender: None,
            handler: None,
            address: 0,
        }
    }

    pub fn set_op(&mut self, op: *const c_char) {
        if !op.is_null() {
            if let Ok(s) = unsafe { CStr::from_ptr(op).to_str() } {
                self.op = s.to_string();
            }
        }
    }

    pub fn add_arg(&mut self, _mode: TtMode, vtype: *const c_char, value: *const c_char) {
        // sophisticated logic would check vtype (int, string)
        // for now, we treat most things as strings unless vtype says "integer"
        let val_str = if !value.is_null() {
            unsafe { CStr::from_ptr(value).to_string_lossy().to_string() }
        } else {
            String::new()
        };

        // Simple heuristic or always string for now as it's most common in CDE
        self.args.push(TtArg::String(val_str));
    }

    pub fn set_file(&mut self, file: *const c_char) {
        if !file.is_null() {
            if let Ok(s) = unsafe { CStr::from_ptr(file).to_str() } {
                self.file = Some(s.to_string());
            }
        }
    }
}
