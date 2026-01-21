#ifndef DTHELP_ENGINE_H
#define DTHELP_ENGINE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct DtHelpEngine DtHelpEngine;

/*
 * Creates a new DtHelpEngine instance.
 * Returns NULL on failure (though currently it panics on allocation fail).
 */
DtHelpEngine *dthelp_engine_new(void);

/*
 * Frees the DtHelpEngine instance.
 * Safe to call with NULL.
 */
void dthelp_engine_free(DtHelpEngine *engine);

/*
 * Sets the text content for the engine to render.
 * The text is expected to be UTF-8 encoded.
 */
void dthelp_engine_set_text(DtHelpEngine *engine, const char *text);

/*
 * Renders the current text into the provided pixel buffer.
 *
 * width: Width of the buffer in pixels.
 * height: Height of the buffer in pixels.
 * pixels: Pointer to a buffer of size width * height * 4 bytes (NB: u32 per
 * pixel). Format is expected to be RGBA or BGRA (depending on backend,
 * currently RGBA8).
 */
void dthelp_engine_render(DtHelpEngine *engine, uint32_t width, uint32_t height,
                          int scroll_y, uint32_t *pixels);

/*
 * Calculates the total height of the content given a specific width.
 * Useful for setting scrollbar ranges.
 */
uint32_t dthelp_engine_get_height(DtHelpEngine *engine, uint32_t width);

/*
 * Calculates the total height of the content given a specific width.
 * Useful for setting scrollbar ranges.
 */
uint32_t dthelp_engine_get_height(DtHelpEngine *engine, uint32_t width);

#ifdef __cplusplus
}
#endif

#endif // DTHELP_ENGINE_H
