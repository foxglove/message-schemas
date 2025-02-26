/*
 * Foxglove SDK
 * https://github.com/foxglove/foxglove-sdk
 */


#ifndef FOXGLOVE_C_H
#define FOXGLOVE_C_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct foxglove_websocket_server foxglove_websocket_server;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create and start a server. The server must later be freed with `foxglove_server_destroy`.
 *
 * # Safety
 * `name` and `host` must be null-terminated strings with valid UTF8.
 */
struct foxglove_websocket_server *foxglove_server_start(const char *name,
                                                        const char *host,
                                                        uint16_t port);

/**
 * Free a server created via `foxglove_server_start`.
 *
 * If the server has not already been stopped, it will be stopped automatically.
 */
void foxglove_server_destroy(struct foxglove_websocket_server *server);

void foxglove_server_stop(struct foxglove_websocket_server *server);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* FOXGLOVE_C_H */
