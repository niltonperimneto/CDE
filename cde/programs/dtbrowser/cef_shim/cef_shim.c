#include <cef/cef_base.h>
#include <malloc.h>
#include <stdio.h>

// Dummy Implementation

int cef_initialize(const cef_settings_t *settings, void *app) {
  fprintf(stderr, "[CEF_SHIM] cef_initialize called\n");
  return 1;
}

void cef_shutdown() { fprintf(stderr, "[CEF_SHIM] cef_shutdown called\n"); }

void cef_do_message_loop_work() {
  // In real CEF, this processes the loop.
  // fprintf(stderr, "."); // Noisy
}

void cef_window_info_set_as_child(cef_window_info_t *info,
                                  unsigned long parent_window, int x, int y,
                                  int width, int height) {
  info->parent_window = parent_window;
  info->x = x;
  info->y = y;
  info->width = width;
  info->height = height;
  fprintf(stderr, "[CEF_SHIM] SetAsChild: Parent=0x%lx\n", parent_window);
}
