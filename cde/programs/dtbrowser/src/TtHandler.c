#include "XmCefBrowser.h"
#include "dtbrowser_ipc.h"
#include <Tt/tt_c.h>
#include <Xm/Xm.h>
#include <stdio.h>

extern Widget browser_widget;

// TtCallback called when data is available on the TT socket
void TtCallback(XtPointer client_data, int *source, XtInputId *id) {
  Tt_message msg_in = tt_message_receive();
  if (msg_in == 0)
    return;

  char *op = tt_message_op(msg_in);
  // We use the first argument as potential URL or Topic ID
  char *arg0 = tt_message_arg_val(msg_in, 0);

  // Use Rust Safety Layer to parse
  IpcCommand cmd = dtbrowser_ipc_parse(op, arg0);

  switch (cmd.type) {
  case IPC_CMD_NAVIGATE:
    if (cmd.url) {
      fprintf(stderr, "[DtBrowser] Rust-IPC Navigate: %s\n", cmd.url);
      XmCefBrowserLoadUrl(browser_widget, cmd.url);
    }
    break;
  case IPC_CMD_SHUTDOWN:
    fprintf(stderr, "[DtBrowser] Rust-IPC Shutdown requested\n");
    exit(0);
    break;
  default:
    // Unknown or unhandled
    break;
  }

  dtbrowser_ipc_free_command(cmd);

  tt_message_destroy(msg_in);
  tt_free(op);
}

void InitializeToolTalk(XtAppContext app) {
  // Initialize Rust layer
  dtbrowser_ipc_init();

  char *procid = tt_open();
  if (tt_ptr_error(procid) != TT_OK) {
    fprintf(stderr, "[DtBrowser] Failed to join ToolTalk: %d\n",
            tt_ptr_error(procid));
    return;
  }

  if (tt_session_join(tt_default_session()) != TT_OK) {
    fprintf(stderr, "[DtBrowser] Failed to join session\n");
  }

  // Register Ptype? Or just pattern?
  // For simplicity, register generic pattern
  Tt_pattern pat = tt_pattern_create();
  tt_pattern_category_set(pat, TT_OBSERVE); // or HANDLE
  tt_pattern_scope_add(pat, TT_SESSION);
  tt_pattern_op_add(pat, "Display_Topic");
  tt_pattern_op_add(pat, "Open_URL");
  tt_pattern_op_add(pat, "Quit");
  tt_pattern_register(pat);

  int fd = tt_fd();
  XtAppAddInput(app, fd, (XtPointer)XtInputReadMask, TtCallback, NULL);

  fprintf(stderr, "[DtBrowser] ToolTalk Initialized\n");
}
