use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, SwashCache};
use libc::c_char;
use tiny_skia::{Paint, PixmapMut, Transform};

mod parser;
pub use parser::*;

pub struct DtHelpEngine {
    font_system: FontSystem,
    swash_cache: SwashCache,
    buffer: Buffer,
}

#[no_mangle]
pub extern "C" fn dthelp_engine_new() -> *mut DtHelpEngine {
    let mut font_system = FontSystem::new();
    let swash_cache = SwashCache::new();
    let metrics = Metrics::new(14.0, 20.0); // Default font size
    let buffer = Buffer::new(&mut font_system, metrics);

    let engine = Box::new(DtHelpEngine {
        font_system,
        swash_cache,
        buffer,
    });
    Box::into_raw(engine)
}

#[no_mangle]
pub extern "C" fn dthelp_engine_free(engine: *mut DtHelpEngine) {
    if !engine.is_null() {
        unsafe {
            let _ = Box::from_raw(engine);
        }
    }
}

#[no_mangle]
pub extern "C" fn dthelp_engine_set_text(engine: *mut DtHelpEngine, text: *const c_char) {
    if engine.is_null() || text.is_null() {
        return;
    }

    let engine = unsafe { &mut *engine };
    let c_str = unsafe { std::ffi::CStr::from_ptr(text) };
    if let Ok(s) = c_str.to_str() {
        engine.buffer.set_text(
            &mut engine.font_system,
            s,
            Attrs::new(),
            cosmic_text::Shaping::Advanced,
        );
    }
}

#[no_mangle]
pub extern "C" fn dthelp_engine_render(
    engine: *mut DtHelpEngine,
    width: u32,
    height: u32,
    scroll_y: i32,
    pixels: *mut u32,
) {
    if engine.is_null() || pixels.is_null() {
        return;
    }
    let engine = unsafe { &mut *engine };

    // Update buffer size
    engine
        .buffer
        .set_size(&mut engine.font_system, width as f32, height as f32);

    // Set scroll
    engine
        .buffer
        .set_scroll(cosmic_text::Scroll::new(0, scroll_y as i32));

    engine
        .buffer
        .shape_until_scroll(&mut engine.font_system, false);

    // Create pixmap wrapper around the provided buffer
    // Safety: Caller must ensure pixels points to a buffer of size width * height * 4
    let data =
        unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, (width * height * 4) as usize) };

    if let Some(mut pixmap) = PixmapMut::from_bytes(data, width, height) {
        pixmap.fill(tiny_skia::Color::WHITE);

        engine.buffer.draw(
            &mut engine.font_system,
            &mut engine.swash_cache,
            Color::rgb(0, 0, 0),
            |x, y, w, h, color| {
                let a = color.a();
                if a > 0 {
                    let paint = Paint {
                        shader: tiny_skia::Shader::SolidColor(tiny_skia::Color::from_rgba8(
                            color.r(),
                            color.g(),
                            color.b(),
                            color.a(),
                        )),
                        ..Paint::default()
                    };

                    // We can try to optimize by creating a tiny rect
                    let rect = tiny_skia::Rect::from_xywh(x as f32, y as f32, w as f32, h as f32);
                    if let Some(r) = rect {
                        pixmap.fill_rect(r, &paint, Transform::identity(), None);
                    }
                }
            },
        );
    }
}

#[no_mangle]
pub extern "C" fn dthelp_engine_get_height(
    engine: *mut DtHelpEngine,
    width: libc::c_int,
) -> libc::c_int {
    if engine.is_null() {
        return 0;
    }
    let engine = unsafe { &mut *engine };

    // Set size to trigger shaping and height calculation
    engine
        .buffer
        .set_size(&mut engine.font_system, width as f32, 10000.0);
    engine
        .buffer
        .shape_until_scroll(&mut engine.font_system, false);

    // Simplified metric:
    let line_height = 20.0; // Estimate
    let count = engine.buffer.lines.len() as f32;
    (count * line_height) as libc::c_int
}
