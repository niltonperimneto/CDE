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
void MakeRootCursor(void);
void MakeBackground(void);
void MakeButtons(void);
void MakeDtlabel(void);
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
void MakeAltDtButtons(void);
static void DebugWidgetResources(Widget w);
static char *GetDisplayName();

/***************************************************************************
 *
 *  Global variables
 *
 ***************************************************************************/

AppInfo appInfo;     /* application resources		   */
Arg argt[100];       /* used for resources			   */
DisplayInfo dpyinfo; /* information about the display	   */
XmString xmstr;      /* used for compound strings		   */
char *errorLogFile;  /* current value of environment var.	   */
int showVerifyError; /* display a Verify() error dialog	   */

char altdtname[MAXPATHLEN];
char altdtclass[MAXPATHLEN];
char altdtkey[MAXPATHLEN];
char altdtkeyclass[MAXPATHLEN];
char altdtstart[MAXPATHLEN];
char altdtstartclass[MAXPATHLEN];
char altdtlogo[MAXPATHLEN];
char altlogoclass[MAXPATHLEN];
char *langenv;
char *logotype;     /* for XrmGetResource()                  */
XrmValue logovalue; /* for XrmGetResource()                    */
char *rmtype;
XrmValue rmvalue;
char *keyrmtype;
XrmValue keyrmvalue;

/******************************************************************************
**
**      WIDGET LAYOUT
**
** toplevel                 "main"                      (toplevel)
**  login_shell              "login_shell"              (overrideShell)
**   table                    "table"                   (DrawingAreaWidget)
**     copyright_msg            "copyright_msg"         (MessageBox)
**     error_message            "error_message"         (MessageBox)
**     help_message             "help_message"          (MessageBox)
**     passwd_message           "passwd_message"        (MessageBox)
**     hostname_message         "hostname_msg"          (MessageBox)
**     matte                   "matte"                  (FormWidget)
**      logo                   "logo"                   (FrameWidget)
**       logo_pixmap            "logo_pixmap"           (LabelGadget)
**      matteFrame         "matteFrame"         (FrameWidget)
**       matte1                "matte1"                 (FormWidget)
**        help_button             "help_button"         (PushButtonGadget)
**        greeting               "greeting"             (LabelGadget)
**        dt_label	         "dt_label"		(LabelGadget)
**        login_form               "login_form"         (FormWidget)
**          login_label              "login_label"      (LabelGadget)
**          login_text               "login_text"       (TextField)
**          passwd_text              "passwd_text"      (TextField)
**        ok_button               "ok_button"           (PushButtonGadget)
**        clear_button            "clear_button"        (PushButtonGadget)
**        options_button                  "options_button"  (DtMenuButtonWidget)
**          options_menu                    "options_menu"      (PopupMenu)
**            options_item[0]         "options_languages"  (CascadeButtonGadget)
**            options_item[1]         "options_sep2"         (SeparatorGadget)
**            options_item[2]         "session_menus"  (CascadeButtonGadget)
**            options_item[3]         "options_sep1"         (SeparatorGadget)
**            options_item[4]         "options_noWindows"    (PushButtonGadget)
**            options_item[5]         "options_restartServer"(PushButtonGadget)
**            options_item[6]         "options_sep1"         (SeparatorGadget)
**            options_item[7]         "options_Copyright"    (PushButtonGadget)
**            session_menu            "session_menu"         (PulldownMenu)
**              options_dt            "options_dt"  (ToggleButtonGadget)
**              options_failsafe      "options_failsafe"  (ToggleButtonGadget)
**            lang_menu               "lang_menu"       (PulldownMenu)
**           (lang items)               (lang items)    (ToggleButtonGadget)
**      ...
**
*/

Widget toplevel;    /* top level shell widget		   */
Widget login_shell; /* shell for the main login widgets.	   */
Widget table;       /* black background for everything	   */
Widget matte;       /* main level form widget		   */
Widget matteFrame;  /* main level form widget		   */
Widget matte1;      /* second level form widget		   */

Widget greeting; /* Welcome message			   */
Widget dt_label; /* Desktop i.e. set in options menu        */

Widget logo1;       /* frame around the Corporate logo	   */
Widget logo_pixmap; /* Corporate logo			   */
Widget logo_shadow; /* drop shadow under the Corporate logo	   */

Widget login_matte; /* bulletin board for login/password	   */
Widget login_form;  /* form containing the login widgets	   */
Widget login_label; /* label to left of login text widget	   */
Widget login_text;  /* login text widget			   */

Widget ok_button;      /* accept name/password text button	   */
Widget clear_button;   /* clear name/password text button	   */
Widget options_button; /* login options button			   */
Widget help_button;    /* help button				   */

Widget copyright_msg = NULL;    /* copyright notice widget		   */
Widget help_message = NULL;     /* the help message box			   */
Widget error_message = NULL;    /* the error message box		   */
Widget hostname_message = NULL; /* the invalid hostname message box	   */
Widget passwd_message = NULL;   /* the expired password message box	   */

Widget options_menu = NULL; /* pop-up menu on options button	   */
Widget options_item[10];    /* items on options pop_up menu	  	   */
Widget options_nowindows;   /* nowindows pane on options pop_up menu   */
Widget options_failsafe;    /* failsafe pane on options pop_up menu	   */
Widget options_dtlite;      /* dtlite  pane on options pop_up menu	   */
Widget *alt_dts;            /* alt_dts  widgets on options pop_up menu */
Widget options_dt;          /* dt regular pane on options pop_up menu */
Widget options_last_dt;     /* user's last dt 			  */

Widget lang_menu = NULL;    /* cascading menu on "Language" option	   */
Widget session_menu = NULL; /* cascading menu on "Session" option     */

/***************************************************************************
 *
 *  Text widget actions and translations
 *
 ***************************************************************************/

/***************************************************************************
 *
 *  Main
 *
 ***************************************************************************/

void MakeRootCursor(void) {
  Cursor vg_cursor;

  vg_cursor = XCreateFontCursor(dpyinfo.dpy, XC_left_ptr);

  XDefineCursor(dpyinfo.dpy, dpyinfo.root, vg_cursor);

  return;
}

/***************************************************************************
 *
 *  MakeBackground
 *
 *  Widgets: login_shell, table, matte
 ***************************************************************************/

void MakeBackground(void) {
  int i;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeBackground:  entered ...");
#endif /* VG_TRACE */
  /*
   * create the login shell widget...
   */

  i = 0;

  /*		CORE resource set					*/
  XtSetArg(argt[i], XmNancestorSensitive, True);
  i++;
  XtSetArg(argt[i], XmNbackgroundPixmap, XmUNSPECIFIED_PIXMAP);
  i++;
  XtSetArg(argt[i], XmNborderWidth, 0);
  i++;
  XtSetArg(argt[i], XmNmappedWhenManaged, False);
  i++;
  XtSetArg(argt[i], XmNsensitive, True);
  i++;
  XtSetArg(argt[i], XmNtranslations, NULL);
  i++;

  /*		COMPOSITE resource set					*/
  XtSetArg(argt[i], XmNinsertPosition, NULL);
  i++;

  /*		SHELL resource set (set to avoid interference by user)	*/
  XtSetArg(argt[i], XmNallowShellResize, False);
  i++;
  XtSetArg(argt[i], XmNcreatePopupChildProc, NULL);
  i++;
  XtSetArg(argt[i], XmNgeometry, NULL);
  i++;
  XtSetArg(argt[i], XmNpopupCallback, NULL);
  i++;
  XtSetArg(argt[i], XmNpopdownCallback, NULL);
  i++;
  XtSetArg(argt[i], XmNoverrideRedirect, False);
  i++;
  XtSetArg(argt[i], XmNsaveUnder, False);
  i++;

  login_shell = XtCreatePopupShell("login_shell", transientShellWidgetClass,
                                   toplevel, argt, i);
  XtAddCallback(login_shell, XmNpopupCallback, LayoutCB, NULL);

  /* Fix to display Input Method's status area. */
  XtSetArg(argt[0], XmNheight, dpyinfo.height);
  XtSetValues(login_shell, argt, 1);

  /*
   * create the full-screen drawing area...
   */

  i = InitArg(DrawingA);
  XtSetArg(argt[i], XmNwidth, dpyinfo.width);
  i++;
  XtSetArg(argt[i], XmNheight, dpyinfo.height);
  i++;
  XtSetArg(argt[i], XmNunitType, XmPIXELS);
  i++;

  table = XtCreateManagedWidget("table", xmDrawingAreaWidgetClass, login_shell,
                                argt, i);

  XtAddEventHandler(table, ButtonPressMask, False, RefreshEH, NULL);
  XtAddCallback(table, XmNhelpCallback, ShowDialogCB, (XtPointer)help);

  /*
   * create the main matte...
   */

  i = InitArg(Form);
  /*		      XmNwidth,			(set by user)		*/
  /*		      XmNheight,		(set by user)		*/
  XtSetArg(argt[i], XmNshadowThickness, SHADOW_THICKNESS);
  i++;
  /*
      XtSetArg(argt[i], XmNshadowType,	XmSHADOW_OUT	); i++;
      XtSetArg(argt[i], XmNshadowThickness,	5	); i++;
  */

  matte = XmCreateForm(table, "matte", argt, i);
  XtManageChild(matte);

  i = 0;
  XtSetArg(argt[i], XmNshadowType, XmSHADOW_OUT);
  i++;
  XtSetArg(argt[i], XmNshadowThickness, 2);
  i++;
  XtSetArg(argt[i], XmNtopAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_FORM);
  i++;
  /*
  XtSetArg(argt[i], XmNrightAttachment, XmATTACH_FORM); i++;
  */
  XtSetArg(argt[i], XmNtopOffset, 15);
  i++;
  XtSetArg(argt[i], XmNbottomOffset, 15);
  i++;
  XtSetArg(argt[i], XmNleftOffset, 15);
  i++;
  /*
  XtSetArg(argt[i], XmNrightOffset, 15); i++;
  */
  matteFrame = XmCreateFrame(matte, "matteFrame", argt, i);
  XtManageChild(matteFrame);

  i = 0;
  matte1 = XmCreateForm(matteFrame, "matte1", argt, i);
  XtManageChild(matte1);
}

void MakeAltDtButtons(void) {
  int i, j;
  struct stat statb;
  char *startup_name;
  XrmValue startup_value;
  char temp[MAXPATHLEN] = "\0";
  char *session;
  FILE *ls;
  char lastsess[MAXPATHLEN];
  Widget default_dt = NULL;
  int default_is_custom_dt = True;
  int found_alt_dt = False;
  char *temp_p;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeAltDtButtons:  entered ...");
#endif /* VG_TRACE */

  if (getenv("SESSION_SET") == NULL) {
    default_is_custom_dt = False;
  }

  if ((session = getenv("SESSION")) == NULL) {
    session = "  ";
  }

  if (appInfo.altDts > 0) {
    if ((alt_dts = (Widget *)calloc(appInfo.altDts, sizeof(Widget))) == NULL)
      LogError(ReadCatalog(MC_ERROR_SET, MC_NO_MEMORY, MC_DEF_NO_MEMORY),
               dpyinfo.name);

    for (i = 0; i < appInfo.altDts; ++i) {
      int is_default;

      is_default = FALSE;

      /* alt desktops begin numbering with 1 */
      sprintf(altdtname, "%s%d", "Dtlogin*altDtName", i + 1);
      sprintf(altdtclass, "%s%d", "Dtlogin*AltDtName", i + 1);

      sprintf(altdtkey, "%s%d", "Dtlogin*altDtKey", i + 1);
      sprintf(altdtkeyclass, "%s%d", "Dtlogin*AltDtKey", i + 1);

      sprintf(altdtstart, "%s%d", "Dtlogin*altDtStart", i + 1);
      sprintf(altdtstartclass, "%s%d", "Dtlogin*AltDtStart", i + 1);

      if (XrmGetResource(XtDatabase(dpyinfo.dpy), altdtkey, altdtkeyclass,
                         &keyrmtype, &keyrmvalue) == True) {
        /*
         * remove trailing spaces
         */
        if (strchr(keyrmvalue.addr, ' '))
          temp_p = strtok(keyrmvalue.addr, " ");
        else
          temp_p = keyrmvalue.addr;

        /*
         * Make sure the key file exists.
         */
        if (stat(temp_p, &statb) == 0) {

          j = InitArg(ToggleBG);
          if (XrmGetResource(XtDatabase(dpyinfo.dpy), altdtstart,
                             altdtstartclass, &startup_name,
                             &startup_value) == True) {

            /*
             * remove trailing spaces
             */
            if (strchr(startup_value.addr, ' '))
              snprintf(temp, sizeof(temp), "%s",
                       strtok(startup_value.addr, " "));
            else
              snprintf(temp, sizeof(temp), "%s", startup_value.addr);

            if (default_is_custom_dt)
              if (strcmp(session, temp) == 0) {
                is_default = TRUE;
              }
          } else
            LogError((unsigned char *)"No startup script for altdt %d \n", i);

          if (XrmGetResource(XtDatabase(dpyinfo.dpy), altdtname, altdtclass,
                             &rmtype, &rmvalue) == True) {
            if (!strncmp(rmvalue.addr, DISPLAYNAME, strlen(DISPLAYNAME))) {
              char *host;

              host = GetDisplayName();
              snprintf(temp, sizeof(temp), "%s - %s", host,
                       rmvalue.addr + strlen(DISPLAYNAME));
              xmstr = XmStringCreateLocalized(temp);
            } else {
              xmstr = XmStringCreateLocalized(rmvalue.addr);
            }
          } else {
            LogError((unsigned char
                          *)"Couldn't find the altdtname resource in the db\n");
            sprintf(altdtname, "%s%d", "Alternate Desktop-", i + 1);
            xmstr = XmStringCreateLocalized(altdtname);
          }

          sprintf(altdtlogo, "%s%d", "Dtlogin*altDtLogo", i + 1);
          sprintf(altlogoclass, "%s%d", "Dtlogin*AltDtLogo", i + 1);
          if (XrmGetResource(XtDatabase(dpyinfo.dpy), altdtlogo, altlogoclass,
                             &logotype, &logovalue) == True) {
            XtSetArg(argt[j], XmNuserData, logovalue.addr);
            j++;
          } else {
            XtSetArg(argt[j], XmNuserData, logoInfo.bitmapFile);
            j++;
          }

          XtSetArg(argt[j], XmNlabelString, xmstr);
          j++;
          XtSetArg(argt[j], XmNrecomputeSize, True);
          j++;

          alt_dts[i] =
              XmCreateToggleButtonGadget(session_menu, rmvalue.addr, argt, j);
          XmStringFree(xmstr);
          XtAddCallback(alt_dts[i], XmNvalueChangedCallback, MenuItemCB,
                        (XtPointer)OB_ALT_DTS);
          XtManageChild(alt_dts[i]);
          found_alt_dt = True;

          if (is_default)
            default_dt = alt_dts[i];
        } else
          LogError((unsigned char *)"Couldn't find the keyfile \n");
      } else
        LogError(
            (unsigned char
                 *)"Couldn't find the altkeyfile resource in the database\n");
    }
  }

  if ((appInfo.altDts == 0) || !found_alt_dt)
    XtManageChild(options_dt);

  /*
   * Use the regular desktop if none of the known sessions matched the
   * specified custom session.
   */
  if (default_is_custom_dt && NULL == default_dt) {
    default_dt = options_dt;
    if (found_alt_dt)
      XtManageChild(options_dt);
  }

  /*
   *  [ Failsafe Session ] menu pane...
   */
  i = InitArg(ToggleBG);
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_FS_LABEL, MC_DEF_FS_LABEL);
  XtSetArg(argt[i], XmNuserData, logoInfo.bitmapFile);
  i++;
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;
  XtSetArg(argt[i], XmNrecomputeSize, True);
  i++;
  options_failsafe =
      XmCreateToggleButtonGadget(session_menu, "options_failsafe", argt, i);
  XmStringFree(xmstr);
  XtAddCallback(options_failsafe, XmNvalueChangedCallback, MenuItemCB,
                (XtPointer)OB_FAILSAFE);

  XtManageChild(options_failsafe);

  /*
   * which option to set..
   */
  SetDefaultDt(default_dt);
  SetDtLabelAndIcon();
}

/***************************************************************************
 *
 *  MakeButtons
 *
 *  Widgets:	ok_button, clear_button, options_button, help_button
 ***************************************************************************/

void MakeButtons(void) {
  int i;

  Dimension max_width;  /* maximum width  of a set of widgets	   */
  Dimension max_height; /* maximum height of a set of widgets	   */
  Dimension thick1;     /* defaultButtonShadowThickness */
  Dimension thick2;     /* shadowThickness */

  int origin;  /* horizontal origin for button placement  */
  int spacing; /* spacing between buttons (width/32)      */

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeButtons:  entered ...");
#endif /* VG_TRACE */

  /*
   * create the buttons...
   */

  /* ok button */

  i = InitArg(PushBG);
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNbottomPosition, 95);
  i++;
  XtSetArg(argt[i], XmNtraversalOn, True);
  i++;

  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_OK_LABEL, MC_DEF_OK_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;

  ok_button = XmCreatePushButtonGadget(matte1, "ok_button", argt, i);

  XmStringFree(xmstr);
  XtManageChild(ok_button);

  XtAddCallback(ok_button, XmNactivateCallback, RespondChallengeCB, NULL);

  /* clear button */

  i -= 1;
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_CLEAR_LABEL, MC_DEF_CLEAR_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;

  clear_button = XmCreatePushButtonGadget(matte1, "clear_button", argt, i);

  XmStringFree(xmstr);
  XtManageChild(clear_button);
  XtAddCallback(clear_button, XmNactivateCallback, RespondClearCB,
                (XtPointer)0);

  /* help button */

  i -= 1;
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_HELP_LABEL, MC_DEF_HELP_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;

  help_button = XmCreatePushButtonGadget(matte1, "help_button", argt, i);
  XtAddCallback(help_button, XmNactivateCallback, ShowDialogCB,
                (XtPointer)help);
  XmStringFree(xmstr);
  XtManageChild(help_button);

  /* options button */

  i = InitArg(Label);
  XtSetArg(argt[i], XmNbottomAttachment, XmATTACH_POSITION);
  i++;
  XtSetArg(argt[i], XmNbottomPosition, 95);
  i++;
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_OPTIONS_LABEL, MC_DEF_OPTIONS_LABEL);

  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;
  XtSetArg(argt[i], XmNtraversalOn, True);
  i++;
  options_button = DtCreateMenuButton(matte1, "options_button", argt, i);
  XtOverrideTranslations(options_button,
                         XtParseTranslationTable(" <Key> Return: Select()"));

  XtManageChild(options_button);
  XmStringFree(xmstr);

  /*
   *  tell form that ok_button is the default button...
   */

  i = 0;
  XtSetArg(argt[i], XmNdefaultButton, ok_button);
  i++;
  XtSetValues(matte1, argt, i);

  /*
   * make all buttons *look* the same size...
   */

  max_width = max_height = 0;
  GetBiggest(ok_button, &max_width, &max_height);
  GetBiggest(clear_button, &max_width, &max_height);
  GetBiggest(options_button, &max_width, &max_height);
  GetBiggest(help_button, &max_width, &max_height);

  if ((int)max_width < MIN_BUTTON_SIZE)
    max_width = MIN_BUTTON_SIZE;

  i = 0;
  XtSetArg(argt[i], XmNdefaultButtonShadowThickness, &thick1);
  i++;
  XtSetArg(argt[i], XmNshadowThickness, &thick2);
  i++;
  XtGetValues(ok_button, argt, i);
  thick1 *= 4;
  thick1 += thick2;

  i = 0;
  XtSetArg(argt[i], XmNwidth, max_width);
  i++;
  XtSetArg(argt[i], XmNheight, max_height);
  i++;
  XtSetArg(argt[i], XmNrecomputeSize, False);
  i++;
  XtSetArg(argt[i], XmNbottomOffset, thick1);
  i++;
  XtSetValues(options_button, argt, i);

  i = 0;
  XtSetArg(argt[i], XmNwidth, max_width + 2 * thick1);
  i++;
  XtSetArg(argt[i], XmNheight, max_height + 2 * thick1);
  i++;
  XtSetArg(argt[i], XmNrecomputeSize, False);
  i++;

  XtSetValues(ok_button, argt, i);
  XtSetValues(clear_button, argt, i);
  XtSetValues(help_button, argt, i);
}

/***************************************************************************
 *
 *  MakeDialog
 *
 *  Widgets: error_message, help_message, copyright_msg, hostname_message,
 *	     passwd_message
 ***************************************************************************/

void MakeDialog(DialogType dtype) {
  int i, j;

  int width;

  FILE *fp, *fopen();
  char buffer[128];

  Widget w = NULL, text;
  Dimension txt_width, txt_height;
  XmString ok, cancel, nw, sv;

  Widget tlev; /* JET - warning, there be dragons here */
  unsigned int dpwidth, dpheight, xorg, yorg;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeDialog:  entered ...");
#endif /* VG_TRACE */
       /*
        *  do things common to all dialogs...
        */

#ifdef USE_XINERAMA
  /* get info on prefered screen */
  if (!_DtXineramaGetScreen(
          dpyinfo.DtXineramaInfo, appInfo.xineramaPreferredScreen, &dpwidth,
          &dpheight, &xorg,
          &yorg)) { /* no joy here either - setup for normal */
    dpwidth = dpyinfo.width;
    dpheight = dpyinfo.height;
    xorg = yorg = 0;
  }
  /* else, should be setup properly */
  XWarpPointer(dpyinfo.dpy, None, dpyinfo.root, 0, 0, 0, 0, dpwidth / 2,
               dpheight / 2);
#else /* no Xinerama */
  dpwidth = dpyinfo.width;
  dpheight = dpyinfo.height;
  xorg = yorg = 0;
#endif

  ok = ReadCatalogXms(MC_LABEL_SET, MC_OK_LABEL, MC_DEF_OK_LABEL);
  cancel = ReadCatalogXms(MC_LABEL_SET, MC_CANCEL_LABEL, MC_DEF_CANCEL_LABEL);

  i = InitArg(MessageBox);
  XtSetArg(argt[i], XmNmarginHeight, MBOX_MARGIN_HEIGHT);
  i++;
  XtSetArg(argt[i], XmNmarginWidth, MBOX_MARGIN_WIDTH);
  i++;
  XtSetArg(argt[i], XmNshadowThickness, SHADOW_THICKNESS);
  i++;
  XtSetArg(argt[i], XmNokLabelString, ok);
  i++;
  XtSetArg(argt[i], XmNcancelLabelString, cancel);
  i++;
  XtSetArg(argt[i], XmNnoResize, False);
  i++;
  XtSetArg(argt[i], XmNresizePolicy, XmRESIZE_ANY);
  i++;

  /*
   *  create the various dialogs...
   */

  /* JET - check the matte widget, and if non-null, well use that as
   * the parent for dialogs.  Otherwise use table (the original
   * toplevel widget for this func).  This is useful for Xinerama so
   * that child dialogs are centered on the matte, and not the whole
   * SLS screen.
   */

  if (matte != (Widget)NULL)
    tlev = matte;
  else
    tlev = table;

  switch (dtype) {

  case error:
    xmstr = ReadCatalogXms(MC_ERROR_SET, MC_LOGIN, "");
    XtSetArg(argt[i], XmNmessageString, xmstr);
    i++;

    w = XmCreateErrorDialog(tlev, "error_message", argt, i);
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_CANCEL_BUTTON));
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_HELP_BUTTON));

    error_message = w;
    break;

  case help:

    txt_width = (dpwidth > 850) ? 800 : dpwidth - 50;
    txt_height = (dpheight > 900) ? 600 : dpheight - 300;

    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_HELP_LABEL, MC_DEF_HELP_LABEL);
    XtSetArg(argt[i], XmNmessageString, xmstr);
    i++;

    w = XmCreateInformationDialog(tlev, "help_message", argt, i);
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_CANCEL_BUTTON));
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_HELP_BUTTON));

    i = InitArg(Text);
    XtSetArg(argt[i], XmNheight, txt_height);
    i++;
    XtSetArg(argt[i], XmNwidth, txt_width);
    i++;
    XtSetArg(argt[i], XmNeditMode, XmMULTI_LINE_EDIT);
    i++;
    XtSetArg(argt[i], XmNscrollBarDisplayPolicy, XmAS_NEEDED);
    i++;
    XtSetArg(argt[i], XmNscrollingPolicy, XmAUTOMATIC);
    i++;
    XtSetArg(argt[i], XmNeditable, False);
    i++;
    XtSetArg(argt[i], XmNvalue, ReadCatalog(MC_HELP_SET, MC_HELP, MC_DEF_HELP));
    i++;
    text = XmCreateScrolledText(w, "help_message_text", argt, i);

    XtManageChild(text);
    XtManageChild(w);
    help_message = w;
    break;

  case copyright:
    if ((fp = fopen(COPYRIGHT, "r")) == NULL)
      xmstr = XmStringCreate("Cannot open copyright file '/etc/copyright'.",
                             XmFONTLIST_DEFAULT_TAG);
    else {
      xmstr = (XmString)NULL;

      while (fgets(buffer, 128, fp) != NULL) {
        j = strlen(buffer);
        if (buffer[j - 1] == '\n')
          buffer[j - 1] = '\0';

        if (xmstr != NULL)
          xmstr = XmStringConcat(xmstr, XmStringSeparatorCreate());

        xmstr = XmStringConcat(xmstr,
                               XmStringCreate(buffer, XmFONTLIST_DEFAULT_TAG));
      }
      fclose(fp);
    }

    XtSetArg(argt[i], XmNmessageString, xmstr);
    i++;

    w = XmCreateInformationDialog(tlev, "copyright_msg", argt, i);
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_CANCEL_BUTTON));
    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_HELP_BUTTON));

    XtAddCallback(w, XmNokCallback, CopyrightCB, (XtPointer)0);

    copyright_msg = w;
    break;

  case hostname:

    nw = ReadCatalogXms(MC_LABEL_SET, MC_NW_LABEL, MC_DEF_NW_LABEL);
    sv = ReadCatalogXms(MC_LABEL_SET, MC_START_LABEL, MC_DEF_START_LABEL);

    xmstr = ReadCatalogXms(MC_HELP_SET, MC_SYSTEM, MC_DEF_SYSTEM);
    XtSetArg(argt[i], XmNmessageString, xmstr);
    i++;
    XtSetArg(argt[i], XmNokLabelString, nw);
    i++;
    XtSetArg(argt[i], XmNcancelLabelString, sv);
    i++;

    w = XmCreateWarningDialog(tlev, "hostname_msg", argt, i);

    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_HELP_BUTTON));

    XmStringFree(nw);
    XmStringFree(sv);

    hostname_message = w;
    break;

  case expassword:

    xmstr =
        ReadCatalogXms(MC_ERROR_SET, MC_PASSWD_EXPIRED, MC_DEF_PASSWD_EXPIRED);
    XtSetArg(argt[i], XmNmessageString, xmstr);
    i++;

    w = XmCreateQuestionDialog(tlev, "password_msg", argt, i);

    XtUnmanageChild(XmMessageBoxGetChild(w, XmDIALOG_HELP_BUTTON));

    passwd_message = w;
    break;
  }

  /*
   *  finish up...
   */

  switch (dtype) {
  case error:
  case hostname:
  case expassword:
    XtAddCallback(w, XmNokCallback, RespondDialogCB, NULL);
    XtAddCallback(w, XmNcancelCallback, RespondDialogCB, NULL);
    break;
  }

  XtSetArg(argt[0], XmNdialogStyle, XmDIALOG_APPLICATION_MODAL);
  i++;
  if (w) {
    XtSetValues(w, argt, 1);
  }

  XmStringFree(xmstr);
  XmStringFree(ok);
  XmStringFree(cancel);

  /*
   *  adjust the width of the "ok" button on the dialogs...
   */

  width = (dtype == hostname ? FromMM(4000) : MIN_BUTTON_SIZE);

  i = 0;
  XtSetArg(argt[i], XmNrecomputeSize, False);
  i++;
  XtSetArg(argt[i], XmNwidth, width);
  i++;

  XtSetValues(XmMessageBoxGetChild(w, XmDIALOG_OK_BUTTON), argt, i);
}

/***************************************************************************
 *
 *  MakeDtlabel
 *
 *  Widgets:	dt_label
 ***************************************************************************/

void MakeDtlabel(void) {
  int i;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeDtlabel:  entered ...");
#endif /* VG_TRACE */

  i = InitArg(LabelG);
  XtSetArg(argt[i], XmNtraversalOn, False);
  i++;
  XtSetArg(argt[i], XmNtopAttachment, XmATTACH_WIDGET);
  i++;
  XtSetArg(argt[i], XmNleftAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNrightAttachment, XmATTACH_FORM);
  i++;
  XtSetArg(argt[i], XmNalignment, XmALIGNMENT_CENTER);
  i++;
  XtSetArg(argt[i], XmNtopWidget, greeting);
  i++;

  dt_label = XmCreateLabel(matte1, "dt_label", argt, i);
  XtManageChild(dt_label);
}

/***************************************************************************
 *
 *  MakeGreeting
 *
 *  Widgets:	greeting
 ***************************************************************************/

void MakeOptionsMenu(void) {
  int i, j, k;

  struct stat statb;

#ifdef VG_TRACE
  vg_TRACE_EXECUTION("MakeOptionsMenu:  entered ...");
#endif /* VG_TRACE */

  /*
   * get the built-in pop_up menu from the DtMenuButton...
   */

  XtVaGetValues(options_button, DtNsubMenuId, &options_menu, NULL);

  /*
   *  create language cascade menus...
   */

  if (lang_menu == NULL)
    MakeLangMenu();

  /*
   *  create first level menu items...
   */
  j = 0;

  /*
   *  build [ Language ] menu pane if there are languages to choose from...
   */
  if (lang_menu != NULL) {
    /*
     *  [ Language ] menu pane...
     *  attach language cascade menu to this pane
     */
    i = InitArg(CascadeBG);
    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_LANG_LABEL, MC_DEF_LANG_LABEL);
    XtSetArg(argt[i], XmNlabelString, xmstr);
    i++;
    XtSetArg(argt[i], XmNsubMenuId, lang_menu);
    i++;
    XtSetArg(argt[i], XmNrecomputeSize, True);
    i++;
    options_item[j] =
        XmCreateCascadeButtonGadget(options_menu, "options_languages", argt, i);
    XmStringFree(xmstr);
    j++;

    /*
     *  separator...
     */
    i = InitArg(SeparatorG);
    options_item[j] =
        XmCreateSeparatorGadget(options_menu, "options_sep2", argt, i);
    j++;
  }

  if (session_menu == NULL) {
    session_menu = XmCreatePulldownMenu(options_menu, "session_menu", NULL, 0);

    /*
     *  [ Dt "Reg" ] menu pane...
     */
    i = k = InitArg(ToggleBG);
    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_DT_LABEL, MC_DEF_DT_LABEL);
    XtSetArg(argt[i], XmNlabelString, xmstr);
    i++;
    XtSetArg(argt[i], XmNrecomputeSize, True);
    i++;
    XtSetArg(argt[i], XmNuserData, logoInfo.bitmapFile);
    i++;

    options_dt =
        XmCreateToggleButtonGadget(session_menu, "options_dt", argt, i);
    XmStringFree(xmstr);
    XtAddCallback(options_dt, XmNvalueChangedCallback, MenuItemCB,
                  (XtPointer)OB_DT);
    /*XtManageChild(options_dt);   */
    /*
     *  [ Dt Lite ] menu pane...
     */
    i = k;
    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_DTLITE_LABEL, MC_DEF_DTLITE_LABEL);
    XtSetArg(argt[i], XmNlabelString, xmstr);
    i++;
    XtSetArg(argt[i], XmNrecomputeSize, True);
    i++;

    options_dtlite =
        XmCreateToggleButtonGadget(session_menu, "options_dtlite", argt, i);
    XmStringFree(xmstr);
    XtAddCallback(options_dtlite, XmNvalueChangedCallback, MenuItemCB,
                  (XtPointer)OB_DTLITE);
  }

  if (session_menu != NULL) {
    /*
     *  [ Language ] menu pane...
     *  attach language cascade menu to this pane
     */
    i = InitArg(CascadeBG);
    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_SES_LABEL, MC_DEF_SES_LABEL);
    XtSetArg(argt[i], XmNlabelString, xmstr);
    i++;
    XtSetArg(argt[i], XmNsubMenuId, session_menu);
    i++;
    XtSetArg(argt[i], XmNrecomputeSize, True);
    i++;
    options_item[j] =
        XmCreateCascadeButtonGadget(options_menu, "session_menus", argt, i);
    XmStringFree(xmstr);
    j++;

    /*
     *  separator...
     */
    i = InitArg(SeparatorG);
    options_item[j] =
        XmCreateSeparatorGadget(options_menu, "options_sep1", argt, i);
    j++;
  }

#if 0
    /*
     *  [ No Windows ] menu pane...
     */
    i = k = InitArg(PushBG);
    xmstr = ReadCatalogXms(MC_LABEL_SET, MC_NW_LABEL, MC_DEF_NW_LABEL);
    XtSetArg(argt[i], XmNlabelString,                   xmstr           ); i++;
    options_item[j] = options_nowindows
                      = XmCreatePushButtonGadget(options_menu,
                                                 "options_noWindows",
                                                 argt, i);
    XmStringFree(xmstr);
    XtAddCallback(options_item[j], XmNactivateCallback,
                  MenuItemCB, (XtPointer) OB_NO_WINDOWS);

    if (getenv(LOCATION) == NULL || strcmp(getenv(LOCATION), "local") != 0 )
        XtSetSensitive(options_item[j], False);
    j++;
#endif

  /*
   *  [ Restart Server ] menu pane...
   */
  i = k = InitArg(PushBG);
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_RS_LABEL, MC_DEF_RS_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;
  options_item[j] =
      XmCreatePushButtonGadget(options_menu, "options_restartServer", argt, i);
  XmStringFree(xmstr);
  XtAddCallback(options_item[j], XmNactivateCallback, MenuItemCB,
                (XtPointer)OB_RESTART_SERVER);
  j++;

#ifdef copyright_option
  /*
   *  separator...
   */
  i = InitArg(SeparatorG);
  options_item[j] =
      XmCreateSeparatorGadget(options_menu, "options_sep1", argt, i);
  j++;

  /*
   *  [ Copyright ] menu pane...
   */
  i = k = InitArg(PushBG);
  xmstr = ReadCatalogXms(MC_LABEL_SET, MC_COPY_LABEL, MC_DEF_COPY_LABEL);
  XtSetArg(argt[i], XmNlabelString, xmstr);
  i++;
  options_item[j] =
      XmCreatePushButtonGadget(options_menu, "options_copyright", argt, i);
  XmStringFree(xmstr);
  XtAddCallback(options_item[j], XmNactivateCallback, MenuItemCB,
                (XtPointer)OB_COPYRIGHT);
  j++;
#endif

  /*
   *  manage the [Options] menu...
   */
  XtManageChildren(options_item, j);

  /*
   *  If the DT Lite Session Manager is not available, remove the DT Lite
   *  and DT menu panes. The actual widgets must still be created since
   *  other code (ex. MenuItemCB()) tries to obtain some of their resources.
   */

  if (stat(DTLITESESSION, &statb) != 0 ||
      ((statb.st_mode & S_IXOTH) != S_IXOTH)) {

    XtUnmanageChild(options_dtlite);
    /*
            XtUnmanageChild(options_dt);
    */
  }

  if (getenv(PINGINTERVAL) != NULL)
    XtUnmanageChild(options_nowindows);
}

/***************************************************************************
 *
 *  MyInsert
 *
 *  Local self-insert action for the text widget. The default action
 *  discards control characters, which are allowed in password.
 ***************************************************************************/

void MakeOptionsProc(XtPointer data, XtIntervalId *id) {

  if (options_menu == NULL)
    MakeOptionsMenu();

  return;
}

/***************************************************************************
 * GetDisplayName (void) - transform the display name into a "short"
 *   host name that is used to create a display-specific session.
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

/***************************************************************************
 *
 *  Terminate
 *
 *  Catch a SIGTERM and unmanage display
 ***************************************************************************/
