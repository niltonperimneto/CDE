#ifndef DTBROWSER_SERVER_H
#define DTBROWSER_SERVER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Starts the embedded HTTP server on a random loopback port.
 * Returns the port number (e.g. 8080).
 * Returns 0 on failure.
 */
uint16_t dtbrowser_server_start(const char *root_dir);

/*
 * Stops the server.
 */
void dtbrowser_server_stop(void);

#ifdef __cplusplus
}
#endif

#endif
