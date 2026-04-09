use crate::{TtClass, TtMode, TtScope};
use libc::{c_char, c_int};
use std::ffi::CStr;

// ---------------------------------------------------------------------------
// Argument value (type-only, no mode).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TtArgValue {
    Int(i32),
    String(String),
    Bytes(Vec<u8>),
}

// ---------------------------------------------------------------------------
// TtArg — a single ToolTalk message argument with its transfer mode.
//
// CDE Tt_mode values (from tt_c.h):
//   TT_MODE_UNDEFINED = 0
//   TT_IN             = 1   (input to the handler)
//   TT_OUT            = 2   (output from the handler)
//   TT_INOUT          = 3   (input and output)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TtArg {
    /// Raw Tt_mode integer (1=TT_IN, 2=TT_OUT, 3=TT_INOUT, 0=undefined→"in").
    pub mode: i32,
    pub value: TtArgValue,
}

impl TtArg {
    pub fn int(mode: i32, v: i32) -> Self {
        Self { mode, value: TtArgValue::Int(v) }
    }

    pub fn string(mode: i32, s: String) -> Self {
        Self { mode, value: TtArgValue::String(s) }
    }

    pub fn bytes(mode: i32, b: Vec<u8>) -> Self {
        Self { mode, value: TtArgValue::Bytes(b) }
    }

    /// Map a raw Tt_mode integer to the wire string used in D-Bus `a(sss)`.
    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            2 => "out",
            3 => "inout",
            _ => "in", // TT_IN = 1; TT_MODE_UNDEFINED = 0 → treat as "in"
        }
    }

    /// Reconstruct a `TtArg` from the three-element wire encoding
    /// `(mode_str, vtype, value)` received in a `MessageDelivered` signal.
    ///
    /// Returns `None` if `vtype` is unrecognised or the value can't be decoded.
    pub fn from_encoded(mode_str: &str, vtype: &str, value: &str) -> Option<Self> {
        let mode = match mode_str {
            "out" => 2,
            "inout" => 3,
            _ => 1, // "in" or anything unexpected → TT_IN
        };
        match vtype {
            "int" => value.parse::<i32>().ok().map(|v| Self::int(mode, v)),
            "string" => Some(Self::string(mode, value.to_owned())),
            "bytes" => Some(Self::bytes(mode, hex_decode(value))),
            _ => None,
        }
    }
}

/// Decode a lowercase hex string back into raw bytes.
///
/// Odd-length or invalid hex digits produce a best-effort partial result
/// (bad nibbles are treated as 0).  This is intentional: the encoding side
/// always produces well-formed hex; an invalid input here implies a bug in
/// the sender, and we prefer to surface a garbled message over panicking.
fn hex_decode(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi = nibble(pair[0]);
            let lo = if pair.len() > 1 { nibble(pair[1]) } else { 0 };
            (hi << 4) | lo
        })
        .collect()
}

fn nibble(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// TtMessage
// ---------------------------------------------------------------------------

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

    /// Add an argument, preserving the caller-supplied `mode` and parsing
    /// `vtype` to select the correct `TtArgValue` variant.
    ///
    /// Recognised vtypes:
    ///   - `"int"` / `"integer"` → parse `value` as decimal i32 (falls back to String)
    ///   - `"bytes"`             → raw bytes from the value pointer
    ///   - anything else         → treat as a UTF-8 string (the most common CDE case)
    pub fn add_arg(&mut self, mode: TtMode, vtype: *const c_char, value: *const c_char) {
        let vtype_str: String = if !vtype.is_null() {
            unsafe { CStr::from_ptr(vtype).to_string_lossy().to_string() }
        } else {
            "string".to_owned()
        };

        let val_str: String = if !value.is_null() {
            unsafe { CStr::from_ptr(value).to_string_lossy().to_string() }
        } else {
            String::new()
        };

        let arg = match vtype_str.as_str() {
            "int" | "integer" => match val_str.parse::<i32>() {
                Ok(v) => TtArg::int(mode, v),
                Err(_) => TtArg::string(mode, val_str),
            },
            "bytes" => TtArg::bytes(mode, val_str.into_bytes()),
            _ => TtArg::string(mode, val_str),
        };

        self.args.push(arg);
    }

    pub fn set_file(&mut self, file: *const c_char) {
        if !file.is_null() {
            if let Ok(s) = unsafe { CStr::from_ptr(file).to_str() } {
                self.file = Some(s.to_string());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Wire encoding
// ---------------------------------------------------------------------------

/// Encode a slice of `TtArg` values into the D-Bus wire representation used
/// by the `org.cde.ToolTalk.SendMessage` method and `MessageDelivered` signal.
///
/// Each argument becomes a `(mode, vtype, value)` string triple (`a(sss)`):
///
/// - `Int(n)`    → `("in"|"out"|"inout", "int",    "<decimal>")`
/// - `String(s)` → `("in"|...,          "string",  s)`
/// - `Bytes(b)`  → `("in"|...,          "bytes",   "<lowercase hex>")`
///
/// This function is `pub` so fuzz targets and unit tests can exercise the
/// encoding without requiring a live D-Bus connection.
pub fn encode_args(args: &[TtArg]) -> Vec<(String, String, String)> {
    args.iter()
        .map(|arg| {
            let mode = arg.mode_str().to_owned();
            match &arg.value {
                TtArgValue::Int(i) => (mode, "int".to_owned(), i.to_string()),
                TtArgValue::String(s) => (mode, "string".to_owned(), s.clone()),
                TtArgValue::Bytes(b) => {
                    let hex: String = b.iter().map(|byte| format!("{:02x}", byte)).collect();
                    (mode, "bytes".to_owned(), hex)
                }
            }
        })
        .collect()
}
