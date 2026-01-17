use crate::{TtCallback, TtCategory, TtScope, TtStatus, TT_OK};
use libc::{c_char, c_void};
use std::ffi::CStr;

#[derive(Debug)]
pub struct TtPattern {
    pub category: TtCategory,
    pub scope: Vec<TtScope>,
    pub ops: Vec<String>,
    pub callbacks: Vec<TtCallback>,
    pub sender: Option<String>,
    // Add more matching criteria as needed
}

impl TtPattern {
    pub fn new() -> Self {
        Self {
            category: 0,
            scope: Vec::new(),
            ops: Vec::new(),
            callbacks: Vec::new(),
            sender: None,
        }
    }

    pub fn add_scope(&mut self, scope: TtScope) {
        self.scope.push(scope);
    }

    pub fn add_op(&mut self, op: *const c_char) {
        if !op.is_null() {
            if let Ok(s) = unsafe { CStr::from_ptr(op).to_str() } {
                self.ops.push(s.to_string());
            }
        }
    }

    pub fn add_callback(&mut self, cb: TtCallback) {
        self.callbacks.push(cb);
    }
}
