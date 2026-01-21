# XmPrintSetup
library callXmPrintSetupsetup and create a Print Shell widget#include <Xm/Print.h>WidgetXmPrintSetupWidgetvideo_widgetScreen*print_screenStringprint_shell_nameArgListargsCardinalnum_argsDESCRIPTIONA function that does the appropriate setting and creates a realized`XmPrintShell`that it returns to the caller. This
function hides the details of theXtto set up a valid print shell
hierarchy for the application. It is also meant
to encourage consistency in the way applications root their print widget
hierarchy.print_screenmust belong to a Display connection that
has already been initialized withXt.Thevideo_widgetis used to get at the application context, application name
and class, andargc/argvstored on theapplicationShellthat roots this widget. If
noapplicationShellis found,NULL argv/argcare used.XmPrintSetupthen creates an unrealizedApplicationShellwith the same name and class as the one given by the video
display, on the print display and on the print screen specified.An`XmPrintShell`is then created as a child of this toplevel shell, usingXtCreatePopupShell, with the nameprint_shell_name, and using theargsprovided. It then realizes and maps the print shell, using`XtPopup`with`XtGrabNone`.This way, application resource files and users can specify
print specific attributes using the following syntax
(ifprint_shell_nameis "Print"):Dtpad.Print*textFontList: somefont
*Print*background:white
*Print*highlightThickness:0

* **`video_widget** 

A video widget to fetch app video data from.
* **`print_screen** 

A print screen on the print display - specifies the screen onto which the new
shell is created.
* **`print_shell_name** 

Specifies the name of the XmPrintShell created on the X Print server.
* **`args** 

Specifies the argument list from which to get the resources for the XmPrintShell.
* **`num_args** 

Specifies the number of arguments in the argument list.RETURN VALUEThe id the`XmPrintShell`widget created on the
X Print Server connection, or NULL if an error has occurred.ERRORS/WARNINGSNone.EXAMPLESFrom the`OK`callback and theSetUpcallback of the primary print dialog widget:static void
printOKCB(Widget, XtPointer call_data, XtPointer client_data)
{
  AppPrint *p = (AppPrint *) client_data;
  DtPrintSetupCallbackStruct *pbs =
                         (XmPrintCallbackStruct *) call_data;

  /* connect if not already done.
     the print dialog callback always provides valid
             printer name, print display and screen
             already initialized: XpInitContext called */
 */
  p->print_shell = XmPrintSetup (widget, pbs->print_screen,
                                              "Print", NULL, 0);

  ...
}SEE ALSO&cdeman.XmPrintShell;,
&cdeman.XmRedisplayWidget;,
&cdeman.XmPrintToFile;,
&cdeman.XmPrintPopupPDM;