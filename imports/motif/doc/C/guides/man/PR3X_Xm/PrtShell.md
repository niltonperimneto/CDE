# XmPrintShell
library callXmPrintShella shell widget class used for printing in Motif#include <Xm/Print.h>BooleanXmIsPrintShellWidgetDESCRIPTIONTheXmPrintShellprovides the Motif application programmer
with an Xt widget oriented API to some of the X Print
resources and a callback to drive the pagination.TheXmPrintShellprovides a simple callback to handle the
pagination logic, and a set of resources to get and set
common printer attributes.If not created on anXPrintconnection,XmPrintShellbehaves
as a regular applicationShell.TheXmPrintShellalso initializes theXpextension event
handling mechanism, by registering an extension selector
that callsXpSelectInputand event dispatcher for print
and attributesXpevents, so applications can useXtInsertEventTypeHandlerto register their own handler
with theXpevents.ArgumentsNoXmCreatefunction is provided, since this is a toplevel
shell, most likely created thru someXtshell creation routine orXmPrintSetup.ClassesXmPrintShellis a subclass ofApplicationShell; it inherits
behavior, resources and traits from all its superclasses.
The class pointer isXmPrintShellWidgetClass.New Resources`XmPrintShell Resource Set``Name``Class``Type``Default``Access``XmNstartJobCallback``XmCCallback``XtCallbackList``NULL``CSG``XmNendJobCallback``XmCCallback``XtCallbackList``NULL``CSG``XmNpageSetupCallback``XmCCallback``XtCallbackList``NULL``CSG``XmNminX``XmCMinX``Dimension``dynamic``G``XmNminY``XmCMinY``Dimension``dynamic``G``XmNmaxX``XmCMaxX``Dimension``dynamic``G``XmNmaxY``XmCMaxY``Dimension``dynamic``G``XmNdefaultPixmapResolution``XmCDefaultPixmapResolution`unsigned short`100``CSG``XmNpdmNotificationCallback``XmCCallback``XtCallbackList``NULL``CSG`

* **`XmNstartJobCallback** 

Specifies the callback driving the beginning of rendering.
It is safe for an application to start rendering after
this callback has been activated.XpStartJobmust be called
to trigger this callback.
* **`XmNendJobCallback** 

Specifies the callback driving the end of rendering.
Notify the client that all rendering has been processed
(whether on print-to-file or regular spool).XpEndJobis called by the print shell to trigger this callback.
* **`XmNpageSetupCallback** 

Specifies the callback driving the page layout. It is safe
for an app to start rendering from this callback even if theXmNstartJobCallbackis not used.
* **`XmNminX, XmNminY, XmNmaxX, XmNmaxY** 

Specify the imageable area of the page in the current
print context.XmPrintShellalso maintains a proper size at all times
by updating its own widget dimension whenever an attribute,
such as resolution or orientation, changes. It is sized in itsInitializeroutine so that the application can rely on a proper size
before the firstStartPagecall is issued.
* **`XmNdefaultPixmapResolution** 

Indicates the resolution in dpi (dot per inch) of
the image files read and converted by Motif for the
widget descendants of this shell. It is used to
determine a scaling ratio to be applied to pixmap
created thru regular pixmap/icon conversion of the
following Widget resources:`XmLabel`.label*Pixmap,`XmIconG`.*IconPixmap`XmToggleB`.selectPixmap,`XmPushBG`.armPixmap,`XmIconG`.*IconMask,`XmMessageBox`.symbolPixmap,`XmContainer`.*StatePixmap, ...Leaving out the pixmap resources being used for
tiling (XmNhighlightPixmap, XmNtopShadowPixmap,
XmNbottomShadowPixmap, XmNbackgroundPixmap, ...)
* **`XmNpdmNotificationCallback** 

A callback notifying the application about the status of the PDM (see
XmPrintPopupPDM). A
XmPrintShellCallbackStruct
is used, with reason:`XmCR_PDM_NONE`: no PDM available on this display for
the named selection (provided in detail)`XmCR_PDM_START_VXAUTH`: the PDM is not authorized to connect to the video display.`XmCR_PDM_START_PXAUTH`: the PDM is not authorized to connect to the print display.`XmCR_PDM_UP`: the PDM is up and running`XmCR_PDM_OK`: the PDM has exited with OK status`XmCR_PDM_CANCEL`: the PDM has exited with CANCEL`XmCR_PDM_START_ERROR`: the PDM cannot start due to some error (usually logged)`XmCR_PDM_EXIT_ERROR`: the PDM has exited with an errorCallback InformationTheXmNstartJobCallback,XmNendJobCallback,XmNpageSetupCallbackandXmNpdmNotificationCallbackoperate on a`XmPrintShellCallbackStruct`, which is defined as follow:typedef struct
{
    int     reason;  /* XmCR_START_JOB, XmCR_END_JOB,
                        XmCR_PAGE_SETUP, XmCR_PDM_* */
    XEvent  *event;
    XPContext print_context;
    Boolean last_page; /* in_out */
    XtPointer detail;
} XmPrintShellCallbackStruct;Additional BehaviorThelast_pagefield
is only meaningful when the reason is`XmCR_PAGE_SETUP`.The page setup callback is called withlast_pageFalseto notify the application that it has
to get its internal layout state ready for the next
page. Typically, a widget based application will change
the content of aLabelshowing the page number, or scroll
the content of theTextwidget.When the application has processed its last page, it
should set thelast_pagefield in the callback struct
toTrue. The callback will be called a last time after
that withlast_pageFalseto notify the application
that it can safely clean-up its internal state (e.g.,
destroy widgets).No drawing should occur from within the callback function
in the application, this is an Exposure event-driven
programming model where widgets render themselves from
their expose methods.The print shell callsXpStartPageafter thepageSetupCallbackreturns, andXpEndPageupon reception ofStartPageNotify.ERRORS/WARNINGS`XmPrintShell`can generate the following warnings:Not connected to a valid X Print Server: behavior undefined.Attempt to set an invalid resolution on a printer: %sAttempt to set an invalid orientation on a printer: %sRETURN VALUENot applicableEXAMPLESPrintOnePageCB(Widget pshell, XtPointer npages,
/*----------*/ XmPrintSetPageCBStruct psp)
{
    static int cur_page = 0;
    cur_page++;

    if (! psp->last_page
        && curPage > 1) /* no need to scroll for the first page */
    {

        XmTextScroll(ptext, prows);  /* get ready for next page */

    } else {    /**** I'm done */

       XtDestroyWidget(pshell);
       XtCloseDisplay(XtDisplay(pshell));
    }

    if (cur_page == (int) n_pages) psp->last_page = True;
}

PrintOKCallback(...)
/*-------------*/
{
    pshell = XmPrintSetup (widget, pbs->print_screen,
                                   "Print", NULL, 0);

    XpStartJob(XtDisplay(pshell), XPSpool);

    /**** here I get the size of the shell, create my widget
          hierarchy: a bulleting board, and then a text widget,
          that I stuff with the video text widget buffer */

    /* get the total number of pages to print */
    /* same code as previous example to get n_pages */

    /****  set up my print callback */
    XtAddCallback(pshell,  XmNpageSetUpCallback,
                           PrintOnePageCB, n_pages);
}Examples ofXmNdefaultPixmapResolutionusage:An application reuses the same image sources it uses for the video interface, in XBM or XPM, to layout on its printed pages. In this case, scaling is seamless.! icon.xpm is 30x30 pixels
    app*dialog.pushb.labelPixmap:icon.xpm
    ! print is 400dpi
    app.print*form.lab.labelPixmap:icon.xpm
    ! 120x120 pixels on the paper (auto scaling)An application provides a new set of image files, for a given printer resolution (say 300). It doesn't want automatic scaling by the toolkit for that resolution, it wants scaling based on these 300dpi images for higher resolution. It creates its print shell inside using the name "printHiRes" and adds the following in its resource file:app.printHiRes.defaultPixmapResolution:300
    ! icon300.xpm is 120x120 pixels
    app.printHiRes*form.lab.labelPixmap:icon300.xpm
    ! 120x120 pixels on the paper (no scaling)This way a printer resolution of 600 will result in a scale of a
300 dpi image by 2 (dpi=600 divided by base=300), while a printer
resolution of 150 (using default print shell name "print") will use
the 100 dpi icon scaled by 1.5 (dpi=150 divided by default base=100).SEE ALSO&cdeman.XmPrintSetup;,
&cdeman.XmRedisplayWidget;,
&cdeman.XmPrintToFile;,
&cdeman.XmPrintPopupPDM;