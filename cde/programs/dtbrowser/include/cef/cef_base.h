#ifndef CEF_SHIM_H
#define CEF_SHIM_H

// Minimal CEF C API definitions for compilation

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Types
typedef struct _cef_string_t {
  char* str;
  size_t length;
  void (*dtor)(char*);
} cef_string_t;

typedef void* cef_window_handle_t;

typedef struct _cef_settings_t {
  size_t size;
  int no_sandbox;
} cef_settings_t;

typedef struct _cef_window_info_t {
    // Platform specific window info
    unsigned long parent_window; 
    int x;
    int y;
    int width;
    int height;
    int windowless_rendering_enabled;
    unsigned long window; // Resulting wrapper window
} cef_window_info_t;

typedef struct _cef_browser_settings_t {
    size_t size;
} cef_browser_settings_t;

// Functions
int cef_initialize(const cef_settings_t* settings, void* app);
void cef_shutdown();
void cef_do_message_loop_work();

// Mock Browser Object
typedef struct _cef_browser_t {
    void* base;
} cef_browser_t;

// Platform Specific
void cef_window_info_set_as_child(cef_window_info_t* info, unsigned long parent_window, int x, int y, int width, int height);

#ifdef __cplusplus
}
#endif

#endif // CEF_SHIM_H
