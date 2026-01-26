/*
 * dtdialog - CDE Dialog Utility
 *
 * A replacement for dtksh dialog scripts.
 * Displays Motif dialogs (error, warning, info, working) via command line.
 *
 * Usage:
 *   dtdialog --error   --title "Title" --message "Message"
 *   dtdialog --warning --title "Title" --message "Message"
 *   dtdialog --info    --title "Title" --message "Message"
 *   dtdialog --working --title "Title" --message "Message"
 *
 * Copyright (c) 2024 CDE Modernization Project
 * Licensed under LGPL 2.1
 */

#include <X11/Intrinsic.h>
#include <Xm/MessageB.h>
#include <Xm/Xm.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
  DIALOG_ERROR,
  DIALOG_WARNING,
  DIALOG_INFO,
  DIALOG_WORKING
} DialogType;

static Widget toplevel;
static Widget dialog;

static void okCallback(Widget w, XtPointer clientData, XtPointer callData) {
  (void)w;
  (void)clientData;
  (void)callData;
  exit(0);
}

static void createDialog(DialogType type, const char *title,
                         const char *message) {
  Widget (*createFunc)(Widget, String, ArgList, Cardinal);
  Arg args[10];
  int n = 0;
  XmString titleStr, messageStr;
  Widget cancelBtn, helpBtn;

  titleStr = XmStringCreateLocalized((char *)title);
  messageStr = XmStringCreateLocalized((char *)message);

  XtSetArg(args[n], XmNdialogTitle, titleStr);
  n++;
  XtSetArg(args[n], XmNmessageString, messageStr);
  n++;
  XtSetArg(args[n], XmNdialogStyle, XmDIALOG_PRIMARY_APPLICATION_MODAL);
  n++;

  switch (type) {
  case DIALOG_ERROR:
    createFunc = XmCreateErrorDialog;
    break;
  case DIALOG_WARNING:
    createFunc = XmCreateWarningDialog;
    break;
  case DIALOG_INFO:
    createFunc = XmCreateInformationDialog;
    break;
  case DIALOG_WORKING:
    createFunc = XmCreateWorkingDialog;
    break;
  default:
    createFunc = XmCreateErrorDialog;
    break;
  }

  dialog = createFunc(toplevel, "dtdialog", args, n);

  /* Remove Cancel and Help buttons */
  cancelBtn = XmMessageBoxGetChild(dialog, XmDIALOG_CANCEL_BUTTON);
  helpBtn = XmMessageBoxGetChild(dialog, XmDIALOG_HELP_BUTTON);
  if (cancelBtn)
    XtUnmanageChild(cancelBtn);
  if (helpBtn)
    XtUnmanageChild(helpBtn);

  XtAddCallback(dialog, XmNokCallback, okCallback, NULL);

  XmStringFree(titleStr);
  XmStringFree(messageStr);

  XtManageChild(dialog);
}

static void printUsage(const char *progname) {
  fprintf(stderr,
          "Usage: %s [--error|--warning|--info|--working] "
          "--title \"Title\" --message \"Message\"\n",
          progname);
  fprintf(stderr, "\nOptions:\n");
  fprintf(stderr, "  --error    Display an error dialog\n");
  fprintf(stderr, "  --warning  Display a warning dialog\n");
  fprintf(stderr, "  --info     Display an information dialog\n");
  fprintf(stderr, "  --working  Display a working/progress dialog\n");
  fprintf(stderr, "  --title    Dialog title\n");
  fprintf(stderr,
          "  --message  Dialog message (can also be positional args)\n");
}

int main(int argc, char *argv[]) {
  XtAppContext appContext;
  DialogType type = DIALOG_ERROR;
  const char *title = "CDE Message";
  const char *message = NULL;
  char messageBuf[4096] = "";
  int i;

  /* Parse arguments */
  for (i = 1; i < argc; i++) {
    if (strcmp(argv[i], "--error") == 0) {
      type = DIALOG_ERROR;
    } else if (strcmp(argv[i], "--warning") == 0) {
      type = DIALOG_WARNING;
    } else if (strcmp(argv[i], "--info") == 0) {
      type = DIALOG_INFO;
    } else if (strcmp(argv[i], "--working") == 0) {
      type = DIALOG_WORKING;
    } else if (strcmp(argv[i], "--title") == 0 && i + 1 < argc) {
      title = argv[++i];
    } else if (strcmp(argv[i], "--message") == 0 && i + 1 < argc) {
      message = argv[++i];
    } else if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
      printUsage(argv[0]);
      return 0;
    } else if (argv[i][0] != '-') {
      /* Positional argument - append to message */
      if (strlen(messageBuf) > 0) {
        strncat(messageBuf, " ", sizeof(messageBuf) - strlen(messageBuf) - 1);
      }
      strncat(messageBuf, argv[i], sizeof(messageBuf) - strlen(messageBuf) - 1);
    }
  }

  /* Use positional args as message if --message not specified */
  if (message == NULL) {
    if (strlen(messageBuf) > 0) {
      message = messageBuf;
    } else {
      fprintf(stderr, "Error: No message specified.\n");
      printUsage(argv[0]);
      return 1;
    }
  }

  /* Initialize Xt */
  toplevel = XtVaAppInitialize(&appContext, "Dtdialog", NULL, 0, &argc, argv,
                               NULL, NULL);

  createDialog(type, title, message);

  XtRealizeWidget(toplevel);
  XtAppMainLoop(appContext);

  return 0;
}
