/*
 * CDE - Common Desktop Environment
 *
 * Copyright (c) 1993-2012, The Open Group. All rights reserved.
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these libraries and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
 */
/* $TOG: vgmain.c /main/19 1998/09/14 18:31:11 mgreess $ */
/*                                                                      *
 * (c) Copyright 1993, 1994 Hewlett-Packard Company                     *
 * (c) Copyright 1993, 1994 International Business Machines Corp.       *
 * (c) Copyright 1993, 1994 Sun Microsystems, Inc.                      *
 * (c) Copyright 1993, 1994 Novell, Inc.                                *
 */

/****************************************************************************
**
**   File:        vgmain.c
**
**   Project:     HP Visual User Environment (DT)
**
**   Description: Main line code for Dtgreet application
**
**                These routines initialize the toolkit, create the widgets,
**		   set up callbacks, and wait for events.
**
**
**   (c) Copyright 1987, 1988, 1989 by Hewlett-Packard Company
**
**
**
****************************************************************************
************************************<+>*************************************/

/***************************************************************************
 *
 *  Includes
 *
 ***************************************************************************/

#include <locale.h>
#include <netdb.h>
#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/param.h>
#include <sys/signal.h>
#include <sys/stat.h>
#include <unistd.h>

#include "vg.h"
#include "vgmsg.h"
#include <Dt/EnvControlP.h>
#include <Dt/MenuButton.h>
#include <X11/Shell.h>
#include <X11/Xfuncs.h>
#include <X11/cursorfont.h>
#include <Xm/CascadeBG.h>
#include <Xm/DragC.h>
#include <Xm/DrawingA.h>
#include <Xm/Form.h>
#include <Xm/Frame.h>
#include <Xm/Label.h>
#include <Xm/LabelG.h>
#include <Xm/MessageB.h>
#include <Xm/PushB.h>
#include <Xm/PushBG.h>
#include <Xm/RowColumn.h>
#include <Xm/SeparatoG.h>
#include <Xm/Text.h>
#include <Xm/TextF.h>
#include <Xm/ToggleBG.h>
#include <Xm/Xm.h>

#ifdef USE_XINERAMA
#include <Dt/DtXinerama.h>
#endif

#if !defined(NL_CAT_LOCALE)
#define NL_CAT_LOCALE 0
#endif

#define LOCALHOST "%LocalHost%"
#define DISPLAYNAME "%DisplayName%"

/***************************************************************************
 *
 *  External declarations
 *
 ***************************************************************************/
extern char password[];     /* pswd string value */
extern int password_length; /* pswd string length */

/***************************************************************************
 *
 *  Procedure declarations
 *
 ***************************************************************************/

static SIGVAL syncTimeout(int arg);
static Widget InitToolKit(int argc, char **argv);
static void MakeRootCursor(void);
void MakeBackground(void);
void MakeButtons(void);
static void MakeDtlabel(void);
static void MakeGreeting(void);
static void MakeLogin(void);
static void MyInsert(Widget w, XEvent *event, char **params,
                     Cardinal *num_params);
static void MyBackspace(Widget w, XEvent *event, char **params,
                        Cardinal *num_params);
static int ErrorHandler(Display *dpy, XErrorEvent *event);
static void xtErrorHandler(String msg);
static void xtWarningHandler(String msg);
void MakeOptionsProc(XtPointer data, XtIntervalId *id);
static SIGVAL Terminate(int arg);
static char *GetLangName(char *label);
static void MakeAltDtButtons(void);
static void DebugWidgetResources(Widget w);
static char *GetDisplayName();

/***************************************************************************
 *
 *  Global variables
 *
 ***************************************************************************/

/*
 *  restored structs specific to dtgreet
 */

static XtActionsRec textActions[] = {
    {"my-insert", (XtActionProc)MyInsert},
    {"my-bksp", (XtActionProc)MyBackspace},
};

static char textEventBindings[] = {
    "Shift <Key>Tab:			prev-tab-group() \n\
  Ctrl <Key>Tab:			next-tab-group() \n\
 <Key>osfEndLine:       		end-of-line() \n\
 <Key>osfBeginLine:     		beginning-of-line() \n\
 ~Shift <Key>osfRight:         		forward-character()\n\
 ~Shift <Key>osfLeft:          		backward-character()\n\
  Ctrl <Key>osfDelete:          	delete-to-end-of-line()\n\
 <Key>osfDelete:                	delete-next-character()\n\
  <Key>osfBackSpace:     		my-bksp()\n\
 <Key>osfActivate:      		activate()\n\
  Ctrl <Key>Return:             	activate()\n\
 <Key>Return:           		activate()\n\
 <Key>:                                 my-insert()\n\
 ~Ctrl ~Shift ~Meta ~Alt<Btn1Down>:     grab-focus() \n\
 <EnterWindow>:                         enter()\n\
 <LeaveWindow>:                         leave()\n\
 <FocusIn>:                             focusIn()\n\
 <FocusOut>:                            focusOut()\n\
 <Unmap>:                               unmap()"};

static XtResource AppResources[] = {
    {"workspaceCursor", "WorkspaceCursor", XtRBoolean, sizeof(Boolean),
     XtOffset(AppInfoPtr, workspaceCursor), XtRImmediate, (caddr_t)False},

    {"labelFont", "LabelFont", XmRFontList, sizeof(XmFontList),
     XtOffset(AppInfoPtr, labelFont), XmRString, "Fixed"},

    {"textFont", "TextFont", XmRFontList, sizeof(XmFontList),
     XtOffset(AppInfoPtr, textFont), XmRString, "Fixed"},

    {"optionsDelay", "OptionsDelay", XtRInt, sizeof(int),
     XtOffset(AppInfoPtr, optionsDelay), XtRImmediate, (XtPointer)0},

    {"altDts", "AltDts", XtRInt, sizeof(int), XtOffset(AppInfoPtr, altDts),
     XtRImmediate, (XtPointer)0},

    {"languageList", "LanguageList", XtRString, sizeof(char *),
     XtOffset(AppInfoPtr, languageList), XtRString, NULL},

#if defined(USE_XINERAMA)
    {"xineramaPreferredScreen", "XineramaPreferredScreen", XtRInt, sizeof(int),
     XtOffset(AppInfoPtr, xineramaPreferredScreen), XtRImmediate, (XtPointer)0},
#endif

#if defined(ENABLE_DYNAMIC_LANGLIST)
    {"languageListCmd", "LanguageListCmd", XtRString, sizeof(char *),
     XtOffset(AppInfoPtr, languageListCmd), XtRString, NULL},
#endif /* ENABLE_DYNAMIC_LANGLIST */

};

typedef struct {
  char *labelString;     /* string for label			   */
  char *persLabelString; /* alternate string for label */
  XmFontList fontList;
} GreetInfo, *GreetInfoPtr;

static GreetInfo greetInfo;

static XtResource greetResources[] = {
    {XmNlabelString, XmCLabelString, XmRString, sizeof(char *),
     XtOffset(GreetInfoPtr, labelString), XtRString, "default"},

    {"persLabelString", "PersLabelString", XmRString, sizeof(char *),
     XtOffset(GreetInfoPtr, persLabelString), XtRString, "default"},

    {XmNfontList, XmCFontList, XmRFontList, sizeof(XmFontList),
     XtOffset(GreetInfoPtr, fontList), XtRString, NULL}};

int main(int argc, char **argv) {

  char *session;
  int i;                  /* index for argt			   */
  char **p;               /* temp pointer to traverse argv	   */
  Boolean nograb = FALSE; /* debugging option to not grab server/key */
  int debug = 0;          /* print debugging output */

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("--------------------- main ------------------------");
#endif /* VG_TRACE */

  setlocale(LC_ALL, "");
  XtSetLanguageProc(NULL, NULL, NULL);
  langenv = getenv("LANG");

  /*
   *  set TERM signal handler...
   */

  (void)signal(SIGTERM, Terminate);

  /*
   *  check some environment variables...
   */

  errorLogFile = getenv(ERRORLOG);

#ifdef sun
  if (getenv("OPENWINHOME") == NULL)
    putenv(OWPATH_ENV);
#endif

  _DtEnvControl(DT_ENV_SET);

  /*
   * set custom error handler for X protocol errors...
   */

  XSetErrorHandler(ErrorHandler);

  /*
   * scan argv looking for display name...
   */

  showVerifyError = 0;

  for (i = argc, p = argv; i > 0; i--, p++) {
    if (strcmp(*p, "-display") == 0) {
      p++;
      i--;
      dpyinfo.name = malloc(strlen(*p) + 1);
      strcpy(dpyinfo.name, *p);
      continue;
    }

    if (strcmp(*p, "-debug") == 0) {
      p++;
      i--;
      debug = atoi(*p);
      continue;
    }

    if (strcmp(*p, "-nograb") == 0) {
      nograb = TRUE;
      continue;
    }

    if (strcmp(*p, "-showerror") == 0) {
      p++;
      i--;
      showVerifyError = atoi(*p);
      continue;
    }
  }

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main: after options.");
#endif /* VG_TRACE */
#ifdef VG_DEBUG
  LogError((unsigned char *)"main:  sleeping %d seconds.\n", debug);
  if (debug) {
    sleep(debug);
  }
#endif /* VG_DEBUG */

  /*
   * initialize the Intrinsics...
   */

  toplevel = InitToolKit(argc, argv);
#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  exited InitToolKit ...");
#endif /* VG_TRACE */

  if (debug) {
    XtSetErrorHandler(xtErrorHandler);
    XtSetWarningHandler(xtWarningHandler);
  }

  /*
   * get information about the display...
   */

  dpyinfo.dpy = XtDisplay(toplevel);
  /*    dpyinfo.name	= "";*/
  dpyinfo.screen = DefaultScreen(dpyinfo.dpy);
  dpyinfo.root = RootWindow(dpyinfo.dpy, dpyinfo.screen);
  dpyinfo.depth = DefaultDepth(dpyinfo.dpy, dpyinfo.screen);
  dpyinfo.width = DisplayWidth(dpyinfo.dpy, dpyinfo.screen);
  dpyinfo.height = DisplayHeight(dpyinfo.dpy, dpyinfo.screen);
  dpyinfo.black_pixel = BlackPixel(dpyinfo.dpy, dpyinfo.screen);
  dpyinfo.visual = DefaultVisual(dpyinfo.dpy, dpyinfo.screen);

  /* JET - for Xinerama, see if the extension */
  /* is available and init accordingly. */

#ifdef USE_XINERAMA

  dpyinfo.DtXineramaInfo = _DtXineramaInit(dpyinfo.dpy);

#ifdef DEBUG
  if (dpyinfo.DtXineramaInfo == NULL) { /* No xinerama, no joy. */
    fprintf(stderr, "### JET VGMAIN: Xinerama NOT available.\n");
  } else {
    fprintf(stderr, "### JET VGMAIN: Xinerama available, scrns = %d\n",
            dpyinfo.DtXineramaInfo->numscreens);
  }
#endif

#endif

  /*
   *  check if any overrides were passed in the argv string...
   */

  for (i = 1; i < argc; i++) {
    switch (i) {

    default:
      break;
    }
  }

  /*
   *  add the unit convertor for resources...
   */

  XtAddConverter(XmRString, XmRUnitType, XmCvtStringToUnitType, NULL, 0);

  /*
   *  get user-specified resources...
   */

  SetResourceDatabase();

  XtGetApplicationResources(toplevel, &appInfo, AppResources,
                            XtNumber(AppResources), NULL, 0);

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  got application resources ...");
#endif /* VG_TRACE */

  /*
   *  build widgets...
   */

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  making UI ...");
#endif              /* VG_TRACE */
  MakeBackground(); /* login_shell, table, matte		  */
  MakeLogo();       /* logo, logo_pixmap, logo_shadow	  */
  MakeGreeting();   /* greeting				  */
  MakeLogin();      /* login_matte ...            		  */
  MakeDtlabel();    /* Show Desktop selection in options  menu*/

  /*
   *  grab the display and keyboard...
   *	moved it from before to after creating text widgets in MakeLogin
   *	RK 01.11.94
   */
  if (!nograb)
    SecureDisplay();

  MakeButtons();         /* ok, clear, options, help buttons	  */
  MakeDialog(copyright); /* copyright dialog	  		  */

  if (appInfo.optionsDelay == 0)
    MakeOptionsMenu(); /* make option_button pop-up menu	  */
  else
    XtAddTimeOut((unsigned long)appInfo.optionsDelay * 1000, MakeOptionsProc,
                 NULL);

  MakeAltDtButtons(); /* make alt desktop buttons, if any  	 */
#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  made UI ...");
#endif /* VG_TRACE */

  /*
   * Add request callback.
   XtAddInput(0, (XtPointer)XtInputReadMask, RequestCB, NULL);
   */

  /*
   *  force the focus to the login_text widget...
   */
  /*
       XtAddEventHandler(login_text, ExposureMask, False,
                          FakeFocusIn, NULL);
  */

  /*
   *  create windows for the widgets...
   */

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  going to realize login_shell ...");
#endif /* VG_TRACE */
  XtRealizeWidget(login_shell);
#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  realized login_shell ...");
#endif /* VG_TRACE */

  /*
   *  miscellaneous stuff...
   *
   *  - turn off keyboard bell
   *  - return root cursor to normal from hourglass
   *  - start pinging the server
   */

  ChangeBell("off");
  if (appInfo.workspaceCursor) {
    MakeRootCursor();
  } else {
    XUndefineCursor(dpyinfo.dpy, dpyinfo.root);
  }
  PingServerCB(NULL, NULL);

  /*
   *  bring up the windows and enter event loop...
   */

  XRaiseWindow(XtDisplay(greeting), XtWindow(greeting));
  /*
XRaiseWindow(XtDisplay(logo_shadow), XtWindow(logo_shadow));
XRaiseWindow(XtDisplay(logo), XtWindow(logo));
  */
  /* XtPopup(login_shell, XtGrabNone); */
  _DtEnvControl(DT_ENV_RESTORE_PRE_DT);

  /*
   * Add request callback.
   */
  sleep(5);
  XtAddInput(0, (XtPointer)XtInputReadMask, RequestCB, NULL);

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("main:  entering XtMainLoop ...");
#endif /* VG_TRACE */
  XtMainLoop();
  exit(0);
}

/***************************************************************************
 *
 *  InitToolKit
 *
 *  initialize the toolkit
 ***************************************************************************/

#define MINTIMEOUT 20

static sigjmp_buf syncJump;

static SIGVAL syncTimeout(int arg)

{
  siglongjmp(syncJump, 1);
}

static Widget InitToolKit(int argc, char **argv) {

  int timeout; /* timeout to initialize toolkit   */
  char *t;
  Widget root;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("InitToolKit: enter ...");
#endif /* VG_TRACE */

  /*
   *  use server grabTimeout as initial value for timeout...
   */

  timeout = ((t = (char *)getenv(GRABTIMEOUT)) == NULL ? 0 : atoi(t));
  timeout += MINTIMEOUT; /* minimum MINTIMEOUT seconds */

  /*
   *  initialize the toolkit. Wrap a timer around it in case the server
   *  is grabbed.
   */

  signal(SIGALRM, syncTimeout);
  if (sigsetjmp(syncJump, 1)) {
    LogError(ReadCatalog(MC_LOG_SET, MC_LOG_NO_DPYINIT, MC_DEF_LOG_NO_DPYINIT),
             dpyinfo.name);
    exit(NOTIFY_RESTART);
  }

  alarm((unsigned)timeout);

  root = XtInitialize("dtlogin", "Dtlogin", NULL, 0, &argc, argv);
  /* Disable Drag and Drop  RK 11.02.93 */
  XtVaSetValues(XmGetXmDisplay(XtDisplay(root)), XmNdragInitiatorProtocolStyle,
                XmDRAG_NONE, NULL);

  alarm(0);
  signal(SIGALRM, SIG_DFL);

  return (root);
}

/***************************************************************************
 *
 *  MakeRootCursor
 *
 *  Widgets: none
 ***************************************************************************/

static void MakeGreeting(void) {
  int i;

  char *greetmsg;
  char host[128];
  char disp[128];
  char *p, *q, *s, *t;
  int newLine = False;
  int skip;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeGreeting:  entered ...");
#endif /* VG_TRACE */

  /*
   *  get the user's greeting preferences...
   */

  XtGetSubresources(table, &greetInfo, "greeting", "Greeting", greetResources,
                    XtNumber(greetResources), NULL, 0);

  /*
   *  get the local hostname...
   */

  gethostname(host, sizeof(host));
  if ((p = strchr(host, '.')) != NULL)
    *p = '\0';

  /*
  **  Get display name (for %DisplayName% substitutions),
  **  reducing "a.b.c.d:0" constructs to shorter "a:0" form.
  */

  strncpy(disp, dpyinfo.name ? dpyinfo.name : (DisplayString(dpyinfo.dpy)),
          127);
  disp[127] = '\0';
  p = strchr(disp, '.');
  t = strchr(disp, ':');
  if (p && t)
    strcpy(p, t);

  /*
   *  use the default string if the user has not specified one...
   */

  if (greetInfo.persLabelString &&
      strcmp(greetInfo.persLabelString, "default") == 0) {
    const char *msg;
    msg = (const char *)ReadCatalog(MC_LABEL_SET, MC_PERS_GREET_LABEL,
                                    MC_DEF_PERS_GREET_LABEL);
    greetInfo.persLabelString = strdup(msg);
  }

  if (greetInfo.labelString && strcmp(greetInfo.labelString, "default") == 0) {

    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_GREET_LABEL, MC_DEF_GREET_LABEL);
    xmstr = XmStringConcat(xmstr, XmStringCreate(" ", XmFONTLIST_DEFAULT_TAG));
    xmstr = XmStringConcat(xmstr, XmStringCreate(host, XmFONTLIST_DEFAULT_TAG));
  } else {
    /*
     *  scan user's message for %LocalHost% token. Replace with hostname
     *  if found...
     */

    if (!greetInfo.labelString || strlen(greetInfo.labelString) == 0 ||
        strcmp(greetInfo.labelString, "None") == 0 ||
        strcmp(greetInfo.labelString, "none") == 0)

      greetmsg = strdup(" ");
    else {
      greetmsg = strdup(greetInfo.labelString);
    }

    s = greetmsg;
    xmstr = (XmString)NULL;

    do {
      q = s;

      /*
       *  scan for a new line character in remaining label string.
       *  If found, work with that substring first...
       */

      if ((p = strchr(q, '\n')) != NULL) {
        *p = '\0';
        newLine = True;
        s = ++p;

        if (*q == '\0') /* handle consecutive newlines */
          q = " ";

      } else {
        newLine = False;
      }

      /*
       *  replace all occurrences of %LocalHost% and %DisplayName%
       *  in the current substring...
       */

      while (1) {
        p = strstr(q, LOCALHOST);
        t = strstr(q, DISPLAYNAME);

        if (p && t) { /* both present? do whichever comes first */
          if (p > t)
            p = NULL;
          else
            t = NULL;
        }
        if (p) { /* replace a %LocalHost% string */
          t = host;
          skip = sizeof(LOCALHOST);
        } else if (t) { /* replace a %DisplayName% string */
          p = t;
          t = disp;
          skip = sizeof(DISPLAYNAME);
        } else /* nothing left to replace */
          break;
        *p = '\0';
        xmstr =
            XmStringConcat(xmstr, XmStringCreate(q, XmFONTLIST_DEFAULT_TAG));
        xmstr =
            XmStringConcat(xmstr, XmStringCreate(t, XmFONTLIST_DEFAULT_TAG));
        q = p + skip - 1;
      }

      if (strlen(q) != 0)
        xmstr =
            XmStringConcat(xmstr, XmStringCreate(q, XmFONTLIST_DEFAULT_TAG));

      /*
       *  add a line separator if this is a multi-line greeting...
       */

      if (newLine == True) {
        xmstr = XmStringConcat(xmstr, XmStringSeparatorCreate());
      }

    } while (newLine == True);

    free(greetmsg);
  }

  /*
   * create the Welcome message...
   */

  i = InitArg(LabelG);
  XtSetArg(argt[i], XmNtraversalOn, False);
  i++;
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNtopAttachment, XmATTACH_POSITION);
  i++;
  /* XtSetArg(argt[i], XmNtopPosition, 15); i++; */
  /* Changed this to accommodate desktop label */
  XtSetArg(argt[i], XmNtopPosition, 9);
  i++;
  XtSetArg(argt[i], XmNrightAttachment, XmATTACH_FORM);
  i++;

  /*
   *  use the user's font if one has been specified, otherwise use
   *  the application's default...
   */

  if (greetInfo.fontList != NULL) {
    XtSetArg(argt[i], XmNfontList, greetInfo.fontList);
    i++;
  }

  greeting = XmCreateLabel(matte1, "greeting", argt, i);
  XtManageChild(greeting);

  XmStringFree(xmstr);
}

/***************************************************************************
 *
 *  MakeLogin
 *
 *  Widgets: login_matte,
 *	     login_form, login_label, login_text
 ***************************************************************************/

static void MakeLogin(void) {
  int i;
  int j;
  LoginTextPtr textdata;
  XtTranslations textTable;
  Widget passwd_text;
  String greetstr;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeLogin:  entered ...");
#endif /* VG_TRACE */

  /*
   *  create the login form
   */

  i = 0;
  XtSetArg(argt[i], XmNshadowThickness, 0);
  i++;
  XtSetArg(argt[i], XmNresizable, False);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNleftOffset, 80);
  i++;
  XtSetArg(argt[i], XmNrightAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNrightOffset, 80);
  i++;
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNbottomPosition, 65);
  i++;
  /*
      XtSetArg(argt[i], XmNresizePolicy, XmRESIZE_ANY); i++;
  */
  XtSetArg(argt[i], XmNallowShellResize, True);
  i++;

  login_form = XmCreateForm(matte1, "login_form", argt, i);
  XtManageChild(login_form);

  /*
   *  create the login text field...
   */

  i = InitArg(Text);
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNrightAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNbottomPosition, 95);
  i++;
  XtSetArg(argt[i], XmNrightPosition, 80);
  i++;
  XtSetArg(argt[i], XmNleftPosition, 20);
  i++;
  XtSetArg(argt[i], XmNselectionArrayCount, 1);
  i++;
  XtSetArg(argt[i], XmNmaxLength, 80);
  i++;
  XtSetArg(argt[i], XmNmappedWhenManaged, False);
  i++;

  textdata = malloc(sizeof(LoginText));
  XtSetArg(argt[i], XmNuserData, textdata);
  i++;

  login_text = XmCreateTextField(login_form, "login_text", argt, i);

  /*
   *  From Human Interface model, Tab key operation should work same on
   *  user field as it does on password field.  Password field is setup
   *  to take Tab key as password data.  HIE model is for user field to
   *  do same.
   */
  XtOverrideTranslations(
      login_text, XtParseTranslationTable(" Shift <Key>Tab: prev-tab-group() \n\
				   Ctrl <Key>Tab: next-tab-group() "));

  XtManageChild(login_text);

  XtAddActions(textActions, 2);
  textTable = XtParseTranslationTable(textEventBindings);

#if 0
    XtSetArg(argt[i], XmNtranslations,          textTable               ); i++;
#endif
  XtSetArg(argt[i], XmNverifyBell, False);
  i++;

  passwd_text = XmCreateTextField(login_form, "passwd_text", argt, i);

  textdata->bEcho = True;
  textdata->noechobuf[0] = '\0';
  textdata->text[0] = passwd_text;
  textdata->text[1] = login_text;

  XtManageChild(passwd_text);
  XtAddCallback(passwd_text, XmNmodifyVerifyCallback, EditPasswdCB, NULL);

  /*
   * Get default greeting string
   */
  i = 0;
  XtSetArg(argt[i], XmNlabelString, &textdata->onGreeting);
  i++;
  XtGetValues(greeting, argt, i);
  textdata->offGreetingFormat = greetInfo.persLabelString;
  textdata->offGreetingUname = NULL;

  /*
   *  create the login labels...
   */

  i = InitArg(LabelG);

  /* modified recomputeSize initial value from False to True, fails
   * when setting longer strings. Manifested as bug ID:1200690.
   */
  XtSetArg(argt[i], XmNrecomputeSize, True);
  i++;

  XtSetArg(argt[i], XmNtraversalOn, False);
  i++;
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_WIDGET);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_OPPOSITE_WIDGET);
  i++;

  /* XtSetArg(argt[i], XmNleftAttachment,	XmATTACH_FORM           ); i++;
     Commented this statement to  align login_label and login_text
      XtSetArg(argt[i], XmNrightAttachment,	XmATTACH_FORM           ); i++;
      XtSetArg(argt[i], XmNalignment,		XmALIGNMENT_CENTER      ); i++;
      XtSetArg(argt[i], XmNbottomOffset,          10                      );
     i++;
  */

  XtSetArg(argt[i], XmNleftWidget, login_text);
  i++;
  XtSetArg(argt[i], XmNbottomWidget, login_text);
  i++;

  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_LOGIN_LABEL, MC_DEF_LOGIN_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;

  login_label = XmCreateLabel(login_form, "login_label", argt, i);
  XtManageChild(login_label);

  XmStringFree(xmstr);
}

/***************************************************************************
 *
 *  MakeOptionsMenu
 *
 *  Widgets: options_menu, options_item[]
 ***************************************************************************/

static void MyInsert(Widget w, XEvent *event, char **params,
                     Cardinal *num_params) {
  char str[32];
  XComposeStatus compstatus;
  int n;

  n = XLookupString((XKeyEvent *)event, str, sizeof(str), (KeySym *)NULL,
                    &compstatus);

  if (n > 0) {
    str[n] = '\0';
    XmTextFieldInsert(w, XmTextFieldGetInsertionPosition(w), str);
  }
}

/***************************************************************************
 *
 *  MyBackspace
 *
 *  Local backspace action for the text widget.
 *  Deletes the last character of the password string in the
 *  widget for each backspace key press, and also does not move the cursor
 *  position in the widget.
 ***************************************************************************/

static void MyBackspace(Widget w, XEvent *event, char **params,
                        Cardinal *num_params) {
  LoginTextPtr textdata;

  textdata = GetLoginTextPtr(w);

  if (textdata && !textdata->bEcho && (int)strlen(textdata->noechobuf) > 0) {
    textdata->noechobuf[strlen(textdata->noechobuf) - 1] = '\0';
  }
}

/***************************************************************************
 *
 *  ErrorHandler
 *
 *  X protocol error handler to override the default
 ***************************************************************************/

static int ErrorHandler(Display *dpy, XErrorEvent *event) { return 0; }

/***************************************************************************
 *
 *  xtErrorHandler
 *
 *  Xt protocol error handler to override the default
 ***************************************************************************/

static void xtErrorHandler(String msg) {
  LogError((unsigned char *)"%s\n", msg);
  exit(NOTIFY_RESTART);
}

/***************************************************************************
 *
 *  xtWarningHandler
 *
 *  Xt protocol error handler to override the default
 ***************************************************************************/

static void xtWarningHandler(String msg) {
  LogError((unsigned char *)"%s\n", msg);
  return;
}

/***************************************************************************
 *
 *  MakeOptionsProc
 *
 *  Timeout routine to build options menu
 ***************************************************************************/

static SIGVAL Terminate(int arg)

{
  if (-1 == write(1, "terminate", 9)) {
    perror(strerror(errno));
  }
  CleanupAndExit(NULL, NOTIFY_ABORT);
}

/***************************************************************************
 *
 *  DebugWidgetResources
 *
 *  Get widget resources
 ***************************************************************************/

typedef struct resource_values {
  int height;
  int width;
  int x;
  int y;
  int rightAttachment;
  int leftAttachment;
  int topAttachment;
  int bottomAttachment;
} ResourceValues;

static void DebugWidgetResources(Widget w)

{
  struct resource_values values;
  int i;

  i = 0;
  bzero((char *)&values, sizeof(values));
  XtSetArg(argt[i], XmNheight, &values.height);
  i++;
  XtSetArg(argt[i], XmNwidth, &values.width);
  i++;
  XtSetArg(argt[i], XmNx, &values.x);
  i++;
  XtSetArg(argt[i], XmNy, &values.y);
  i++;
  XtSetArg(argt[i], XmNrightAttachment, &values.rightAttachment);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, &values.leftAttachment);
  i++;
  XtSetArg(argt[i], XmNtopAttachment, &values.topAttachment);
  i++;
  XtSetArg(argt[i], XmNbottomAttachment, &values.bottomAttachment);
  i++;

  XtGetValues(w, argt, i);
}

/***************************************************************************
 *
 * GetDisplayName (void) - transform the display name into a "short"
 *   host name that is used to create a display-specific session.
 *
 * The display name should match one of the following patterns:
 *
 *   1. host 		(non-qualified)
 *   2. host.domain
 *   3. host:n
 *   4. host:n.s
 *   5. host.domain:n
 *   6. host.domain:n.s
 *
 * Note that 1 and 2 will be used if the display name is actually
 * something like unix:0, local:0 or 0:0
 *
 ***************************************************************************/
static char *GetDisplayName(void) {
  char host[MAXHOSTNAMELEN];
  static char tmp[MAXHOSTNAMELEN + 3];
  char *pch;
  char *col;
  char *dot;

  pch = XDisplayString(dpyinfo.dpy);

  if (!pch || !strncmp(pch, "local:0", 7) || !strncmp(pch, "unix:0", 6) ||
      !strncmp(pch, ":0.0", 4) || !strncmp(pch, ":0", 2)) {
    gethostname(host, MAXHOSTNAMELEN);
    pch = host;
  }

  col = strchr(pch, ':');
  dot = strchr(pch, '.');

  if (!col) {
    if (dot) {
      strncpy(tmp, pch, dot - pch); /* case #2 above */
      tmp[dot - pch] = '\000';
    } else {
      strcpy(tmp, pch); /* case #1 above */
    }
  } else {
    if (!dot || (dot > col)) { /* case #3 and 4 above */
      strncpy(tmp, pch, col - pch);
      tmp[col - pch] = '\000';
    } else { /* case # 5 and 6 above */
      strncpy(tmp, pch, dot - pch);
      tmp[dot - pch] = '\000';
    }
  }

  strcat(tmp, ":0");
  return (tmp);
}
