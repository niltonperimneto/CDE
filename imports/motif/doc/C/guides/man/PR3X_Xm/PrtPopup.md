# XmPrintPopupPDM
library callXmPrintPopupPDMSend a notification for the PDM to be popped up#include <Xm/Print.h>XtEnumXmPrintPopupPDMWidgetprint_shellWidgetvideo_transient_for
## DESCRIPTION
A convenience function that sends a notification to start a
Print Dialog Manager on behalf of the application,XmPrintPopupPDMhides the details of the X selection
mechanism used to notify the PDM that a new dialog must be popped up for this application.XmPrintPopupPDMsends a selection request
to either the print display of the
print shell, or the video display of the
transient_for video widget (depending on
the environment variable`XPDMDISPLAY`,
which can only takes the value "print" or "video"),
asking for the PDM windows to be popped up on behalf
of the app.Return right away with status of`XmPDM_NOTIFY_FAIL`(e.g. if the function couldn't malloc
memory for the selection value, or if`XPDMDISPLAY`is not "print" or "video") or with`XmPDM_NOTIFY_SUCCESS`, which only means a "message" was sent out to the
PDM specified by`XPDMSELECTION`, not that it's already up on the screen yet.In order to know if the PDM is up, or not running,
the application must register aXmNpdmNotificationCallbackwith the Print Shell.XmPrintPopupPDMputs up anInputOnlywindow on top of the dialog, so that
the end user doesn't use the print setup dialog while the PDM is trying to
come up. This window is automatically removed when the shell is
about to call the callback for the first time.

* **`print_shell** 

The Print Shell used for this print job and context.
* **`video_transient_for** 

The video widget dealing with application print setup.RETURN VALUEReturns`XmPDM_NOTIFY_SUCCESS`if the function
was able to send the notification out to the PDM process,`XmPDM_NOTIFY_FAIL`otherwise.ERRORS/WARNINGSNot applicable.EXAMPLESExample of callback from a Print set up dialog box "Setup..." button:PrintSetupCallback(print_dialog...)
/*-------------*/
{
    if (XmPrintPopupPDM (pshell, XtParent(print_dialog)) !=
                                    XmPDM_NOTIFY_SUCCESS) {
        /* some error dialog */
    }
}Example ofXmNpdmNotificationCallbackfrom a Print Shell:pdmNotifyCB(print_shell...)
{
    XmPrintShellCallBackStruct * pr_cb = ...

    switch (pr_cb->reason) {
       case XmCR_PDM_NONE:
           /* no PDM available */
           PostErrorDialog(...);
           break;
       case XmCR_PDM_VXAUTH:
           /* PDM is not authorized ... */
           PostErrorDialog(...);
           break;
       case XmCR_PDM_UP: the PDM is up and running
           /* everything is fine */
           break;
               default: /* other cases */
   }
}SEE ALSO&cdeman.XmPrintSetup;,
&cdeman.XmPrintShell;,
&cdeman.XmRedisplayWidget;,
&cdeman.XmPrintToFile;