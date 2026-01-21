use libc::{c_char, c_int, c_void};
use pulldown_cmark::{Event, Parser, Tag};
use std::ffi::CStr;

pub type ParseCallback = extern "C" fn(ctx: *mut c_void, event_type: c_int, text: *const c_char);

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

#[no_mangle]
pub extern "C" fn dthelp_parse_markdown(
    buffer: *const c_char,
    ctx: *mut c_void,
    callback: ParseCallback,
) {
    if buffer.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(buffer) };
    let markdown_input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    let parser = Parser::new(markdown_input);

    for event in parser {
        match event {
            Event::Start(Tag::Heading(_, _, _)) => {
                (callback)(ctx, EV_HEADER_START, std::ptr::null());
            }
            Event::End(Tag::Heading(_, _, _)) => {
                (callback)(ctx, EV_HEADER_END, std::ptr::null());
            }
            Event::Start(Tag::Paragraph) => {
                (callback)(ctx, EV_PARA_START, std::ptr::null());
            }
            Event::End(Tag::Paragraph) => {
                (callback)(ctx, EV_PARA_END, std::ptr::null());
            }
            Event::Start(Tag::Emphasis) => {
                (callback)(ctx, EV_ITALIC_START, std::ptr::null());
            }
            Event::End(Tag::Emphasis) => {
                (callback)(ctx, EV_ITALIC_END, std::ptr::null());
            }
            Event::Start(Tag::Strong) => {
                (callback)(ctx, EV_BOLD_START, std::ptr::null());
            }
            Event::End(Tag::Strong) => {
                (callback)(ctx, EV_BOLD_END, std::ptr::null());
            }
            Event::Code(text) => {
                (callback)(ctx, EV_CODE_START, std::ptr::null());
                send_text_callback(ctx, callback, &text);
                (callback)(ctx, EV_CODE_END, std::ptr::null());
            }
            Event::Text(text) => {
                send_text_callback(ctx, callback, &text);
            }
            Event::Start(Tag::Link(_, url, _)) => {
                // Fix: removed unused 'title' to suppress warning
                send_text_callback(ctx, callback, &url);
                (callback)(ctx, EV_LINK_START, std::ptr::null());
            }
            Event::End(Tag::Link(_, _, _)) => {
                (callback)(ctx, EV_LINK_END, std::ptr::null());
            }
            _ => {}
        }
    }
}

fn send_text_callback(ctx: *mut c_void, callback: ParseCallback, text: &str) {
    if let Ok(c_string) = std::ffi::CString::new(text) {
        (callback)(ctx, EV_TEXT, c_string.as_ptr());
    }
}

// Added missing function from SetList.c
#[no_mangle]
pub extern "C" fn dthelp_engine_get_height(engine: *mut c_void, width: c_int) -> c_int {
    // Stub implementation
    // In real implementation, this would layout the text and return height.
    // For now, return a dummy height based on some estimation or just a constant.
    // Width is passed to allow wrapping calculation.

    // Simple mock: return 1000 units.
    1000
}
