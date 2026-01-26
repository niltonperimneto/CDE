#include "TtHandler.h"
#include "XmCefBrowser.h"
#include "dtbrowser_server.h"
#include <Xm/MainW.h>
#include <Xm/PushB.h>
#include <Xm/RowColumn.h>
#include <Xm/TextF.h>
#include <Xm/Xm.h>
#include <cef/cef_base.h>
#include <stdio.h>

// Globals
XtAppContext app;
Widget browser_widget;
Widget url_text;

void GoCallback(Widget w, XtPointer client_data, XtPointer call_data) {
  char *url = XmTextFieldGetString(url_text);
  if (url) {
    XmCefBrowserLoadUrl(browser_widget, url);
    XtFree(url);
  }
}

void TimerCallback(XtPointer client_data, XtIntervalId *id) {
  // Drive CEF message loop
  cef_do_message_loop_work();

  // Re-register
  XtAppAddTimeOut(app, 10, TimerCallback, NULL);
}

int main(int argc, char **argv) {
  Widget toplevel, main_window, rowcol, go_btn;

  // Initialize CEF
  cef_settings_t settings = {0};
  settings.no_sandbox = 1;

  if (!cef_initialize(&settings, NULL)) {
    fprintf(stderr, "Failed to initialize CEF\n");
    return 1;
  }

  // Start Rust Help Server
  // In a real CDE install, this would be /usr/dt/appconfig/help or similar
  uint16_t port = dtbrowser_server_start("/usr/dt/appconfig/help");
  char initial_url[256];
  if (port > 0) {
    snprintf(initial_url, sizeof(initial_url), "http://127.0.0.1:%d/index.html",
             port);
    fprintf(stderr, "[DtBrowser] Help Server running on port %d\n", port);
  } else {
    snprintf(initial_url, sizeof(initial_url), "about:blank");
    fprintf(stderr, "[DtBrowser] Failed to start Help Server\n");
  }

  // Initialize Motif/Xt
  XtSetLanguageProc(NULL, NULL, NULL);
  toplevel =
      XtVaAppInitialize(&app, "DtBrowser", NULL, 0, &argc, argv, NULL, NULL);

  // Main Window
  main_window =
      XtVaCreateManagedWidget("mainWindow", xmMainWindowWidgetClass, toplevel,
                              XmNwidth, 800, XmNheight, 600, NULL);

  // Layout
  rowcol = XtVaCreateManagedWidget("rowCol", xmRowColumnWidgetClass,
                                   main_window, NULL);

  // URL Bar
  url_text =
      XtVaCreateManagedWidget("urlText", xmTextFieldWidgetClass, rowcol, NULL);

  go_btn = XtVaCreateManagedWidget("Go", xmPushButtonWidgetClass, rowcol, NULL);
  XtAddCallback(go_btn, XmNactivateCallback, GoCallback, NULL);

  // Browser Widget
  browser_widget = XtVaCreateManagedWidget("browser", xmCefBrowserWidgetClass,
                                           rowcol, XmNurl, initial_url,
                                           XmNwidth, 800, XmNheight, 500, NULL);

  XtRealizeWidget(toplevel);

  // Start CEF Pump
  XtAppAddTimeOut(app, 10, TimerCallback, NULL);

  InitializeToolTalk(app);

  XtAppMainLoop(app);

  // Stop server on exit
  dtbrowser_server_stop();
  cef_shutdown();
  return 0;
}
