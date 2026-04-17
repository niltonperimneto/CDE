#![deny(unsafe_op_in_unsafe_fn)]

//! DtHelp Markdown → event-stream FFI bridge
//!
//! Converts a Markdown buffer into a flat stream of typed events that the
//! legacy C side can consume via a callback pointer.  The design keeps Rust
//! strictly on the parse side and lets the C caller own rendering, which
//! avoids leaking `pulldown-cmark`'s type surface across the FFI boundary.
//!
//! # Safety contract
//!
//! `dthelp_parse_markdown` expects a NUL-terminated UTF-8 string in `buffer`;
//! any `\0` interior bytes produced by the parser (unlikely but possible with
//! pathological input) are silently skipped via `CString::new` failure — we do
//! **not** panic across the FFI boundary.
//!
//! # pulldown-cmark version
//!
//! Pinned to the `0.13.x` API: `Tag::Heading`, `Tag::Link`, etc. became struct
//! variants; `Event::End` now takes a `TagEnd` rather than a `Tag`.  Keep this
//! file in sync when the workspace bumps the crate.

use libc::{c_char, c_int, c_void};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::ffi::CStr;

/// C-callable event handler.
///
/// `event_type` is one of the `EV_*` constants below; `text` is a
/// NUL-terminated UTF-8 pointer that is valid for the duration of the call
/// only (the backing `CString` is dropped as soon as the callback returns).
pub type ParseCallback = extern "C" fn(ctx: *mut c_void, event_type: c_int, text: *const c_char);

// Event-type discriminators.  Keep in sync with the C header
// `cde/lib/DtHelp/rust/dthelp_parse.h`.
const EV_TEXT: c_int = 0;
const EV_HEADER_START: c_int = 1;
const EV_HEADER_END: c_int = 2;
const EV_PARA_START: c_int = 3;
const EV_PARA_END: c_int = 4;
const EV_BOLD_START: c_int = 5;
const EV_BOLD_END: c_int = 6;
const EV_ITALIC_START: c_int = 7;
const EV_ITALIC_END: c_int = 8;
const EV_CODE_START: c_int = 9;
const EV_CODE_END: c_int = 10;
const EV_LINK_START: c_int = 11;
const EV_LINK_END: c_int = 12;

/// Parse a Markdown buffer and stream structural events into a C callback.
///
/// Returns silently if `buffer` is NULL or the buffer is not valid UTF-8.  No
/// Rust panic may unwind across this boundary: the function is written to be
/// infallible in that sense.
#[unsafe(no_mangle)]
pub extern "C" fn dthelp_parse_markdown(
    buffer: *const c_char,
    ctx: *mut c_void,
    callback: ParseCallback,
) {
    if buffer.is_null() {
        return;
    }

    // SAFETY: caller guarantees `buffer` is a NUL-terminated C string.
    let c_str = unsafe { CStr::from_ptr(buffer) };
    let markdown_input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    for event in Parser::new(markdown_input) {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                callback(ctx, EV_HEADER_START, std::ptr::null());
            }
            Event::End(TagEnd::Heading(_)) => {
                callback(ctx, EV_HEADER_END, std::ptr::null());
            }
            Event::Start(Tag::Paragraph) => {
                callback(ctx, EV_PARA_START, std::ptr::null());
            }
            Event::End(TagEnd::Paragraph) => {
                callback(ctx, EV_PARA_END, std::ptr::null());
            }
            Event::Start(Tag::Emphasis) => {
                callback(ctx, EV_ITALIC_START, std::ptr::null());
            }
            Event::End(TagEnd::Emphasis) => {
                callback(ctx, EV_ITALIC_END, std::ptr::null());
            }
            Event::Start(Tag::Strong) => {
                callback(ctx, EV_BOLD_START, std::ptr::null());
            }
            Event::End(TagEnd::Strong) => {
                callback(ctx, EV_BOLD_END, std::ptr::null());
            }
            Event::Code(text) => {
                callback(ctx, EV_CODE_START, std::ptr::null());
                send_text_callback(ctx, callback, &text);
                callback(ctx, EV_CODE_END, std::ptr::null());
            }
            Event::Text(text) => {
                send_text_callback(ctx, callback, &text);
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                send_text_callback(ctx, callback, &dest_url);
                callback(ctx, EV_LINK_START, std::ptr::null());
            }
            Event::End(TagEnd::Link) => {
                callback(ctx, EV_LINK_END, std::ptr::null());
            }
            _ => {}
        }
    }
}

/// Emit a single TEXT event.  Silently drops strings containing interior NULs
/// rather than panicking; the legacy C renderer cannot represent them anyway.
fn send_text_callback(ctx: *mut c_void, callback: ParseCallback, text: &str) {
    if let Ok(c_string) = std::ffi::CString::new(text) {
        callback(ctx, EV_TEXT, c_string.as_ptr());
    }
}

// Added missing function from SetList.c
#[unsafe(no_mangle)]
pub extern "C" fn dthelp_engine_get_height(_engine: *mut c_void, _width: c_int) -> c_int {
    // Stub implementation
    // In real implementation, this would layout the text and return height.
    // For now, return a dummy height based on some estimation or just a constant.
    // Width is passed to allow wrapping calculation.

    // Simple mock: return 1000 units.
    1000
}
