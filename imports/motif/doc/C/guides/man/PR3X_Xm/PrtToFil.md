# XmPrintToFile
library call`XmPrintToFile`Retrieves and saves data that
would normally be printed by the X Print Server.#include <Xm/Print.h>XtEnum`XmPrintToFile`Display*dpyStringfilenameXPFinishProcfinish_procXtPointerclient_dataDESCRIPTIONXmPrintToFilehides the details of X display connection andXpGetDocumentDatato the Motif application programmer.This function is a convenience routine that hides
the details of the X and Xp internals to the
application programmer by calling theXpGetDocumentDatafunction with appropriate save and finish callbacks.This is used in the context of X Printing when
the user has specified the "print-to-file" option from
a regular Print Setup Dialog box.XmPrintToFilefirst tries to open the given filename for writing and returnsFalseif it can't.
Else, it usesXpGetDocumentData, giving it a save proc that writes the
data received in the file and a finish proc that closes the
file or removes it on an unsuccessful termination.
It callsfinish_procat that point, passing it the argument received from the
Xp layer (status == XPGetDocFinishedmeans the file is valid and was closed, otherwise the file was removed).XmPrintToFileis non-blocking; if it returns successfully, it
just means the file was opened successfully, not
that all the data was received.
`dpy`Print display connection.`filename`Name of the file to put the print data in.`finish_proc`Called when all the data has been received.`client_data`Passed with the`finish_proc.RETURN VALUEReturnsFalseif the filename could not be created or opened for writing,Trueotherwise.ERRORS/WARNINGSNot applicableEXAMPLESA typical OK callback from aDtPrintSetupBox:PrintOKCallback(widget...)
/*-------------*/
{   int save_data = XPSpool;

    pshell = XmPrintSetup (widget, pbs->print_screen,
                                   "Print", NULL, 0);

    XtAddCallback(pshell, XmNstartJobCallback, startJobCB, data);

    if (pbs->destination == DtPRINT_TO_FILE)
                 save_data = XPGetData;

    /* start job must precede XpGetDocumentData in XmPrintToFile */
    XpStartJob(XtDisplay(pshell), save_data);
    XFlush(XtDisplay(pshell));  /* maintain the sequence
                                 between startjob and getdocument */

    /* setup print to file */
    if (pbs->destination == DtPRINT_TO_FILE)
        XmPrintToFile(XtDisplay(pshell),
                                 pbs->dest_info, FinishPrintToFile, NULL);
    }

}

static void
startJobCB(Widget, XtPointer call_data, XtPointer client_data)
{
  print(p);   /* rendering happens here */

  XpEndJob(XtDisplay(p->print_shell));

  /* clean up */
  XtDestroyWidget(p->print_shell);
          XtCloseDisplay(XtDisplay(p->print_shell));
}SEE ALSO&cdeman.XmPrintSetup;,
&cdeman.XmPrintShell;,
&cdeman.XmRedisplayWidget;,
&cdeman.XmPrintPopupPDM;