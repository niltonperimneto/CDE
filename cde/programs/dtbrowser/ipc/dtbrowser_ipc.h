#ifndef DTBROWSER_IPC_H
#define DTBROWSER_IPC_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  IPC_CMD_UNKNOWN = 0,
  IPC_CMD_NAVIGATE = 1,
  IPC_CMD_SHUTDOWN = 2
} IpcCommandType;

typedef struct {
  IpcCommandType type;
  char *url;
} IpcCommand;

void dtbrowser_ipc_init(void);
IpcCommand dtbrowser_ipc_parse(const char *op, const char *arg);
void dtbrowser_ipc_free_command(IpcCommand cmd);

#ifdef __cplusplus
}
#endif

#endif
