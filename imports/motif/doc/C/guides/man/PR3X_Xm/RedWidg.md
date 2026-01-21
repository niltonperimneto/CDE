# XmRedisplayWidget
library call`XmRedisplayWidget`Synchronously activates theexposemethod of a widget to draw its content#include <Xm/Xm.h>void`XmRedisplayWidget`WidgetwidgetDESCRIPTIONThis function is a convenience routine that hides the
details of the Xt internals to the application programmer by calling theexposemethod of the given widget with a well formedExposeevent andRegioncorresponding to the total area of the widget. If the widget doesn't have anExposemethod, the function does nothing.This is primarily used in the context of X Printing if the
programming model chosen by the application issynchronous;
that is, it doesn't rely of X Print events for the driving of
page layout but wants to completely control the sequence of rendering requests.XmRedisplayWidgetdoesn't clear the widget window prior to calling theexposemethod, since this is handled by calls toXpStartPage.`widget`The widget to redisplay.RETURN VALUENone.ERRORS/WARNINGSNot applicableEXAMPLESIn the following, a simple application wants to
print the content of a multi-page text widget (similar todtpad).PrintOKCallback(print_dialog...)
/*-------------*/
{
    pshell = XmPrintSetup (print_dialog, pbs->print_screen,
                                   "Print", NULL, 0);

    XpStartJob(XtDisplay(pshell), XPSpool);

    /**** here I realize the shell, get its size, create my widget
     hierarchy: a bulletin board, and then a text widget,
     that I stuff with the video text widget buffer */

    /* get the total number of pages to print */
    XtVaGetValues(ptext, XmNrows, &prows,
                         XmNtotalLines, n_lines, NULL);
    n_pages = n_lines / prows;

    /***** now print the pages in a loop */

    for (cur_page=0; cur_page != n_pages; cur_page++) {

               XpStartPage(XtDisplay(pshell), XtWindow(pshell), False);
               XmRedisplayWidget(ptext);  /* do the drawing */
               XpEndPage(XtDisplay(pshell));

        XmTextScroll(ptext, prows);  /* get ready for next page */
    }

    /***** I'm done */
    XpEndJob(XtDisplay(pshell));

}Of course, one could change the above code to include it in afork()branch so that the main program is not blocked while
printing is going on. Another way to achieve a
"print-in-the-background" effect is to use an Xt workproc. Using the
same sample application, that gives us:Boolean
PrintOnePageWP(XtPointer npages) /* workproc */
/*-------------*/
{
    static int cur_page = 0;
    cur_page++;

    XpStartPage(XtDisplay(pshell), XtWindow(pshell), False);
    XmRedisplayWidget(ptext);  /* do the drawing */
    XpEndPage(XtDisplay(pshell));

    XmTextScroll(ptext, prows);  /*  get ready for next page */

    if (cur_page == n_pages) { /***** I'm done */
        XpEndJob(XtDisplay(pshell));

        XtDestroyWidget(pshell);
        XtCloseDisplay(XtDisplay(pshell));
    }

    return (cur_page == n_pages);
}

PrintOKCallback(...)
/*-------------*/
{
    pshell = XmPrintSetup (widget, pbs->print_screen,
                                   "Print", NULL, 0);

    XpStartJob(XtDisplay(pshell), XPSpool);

    /**** here I get the size of the shell, create my widget
          hierarchy: a bulletin board, and then a text widget,
                  that I stuff with the video text widget buffer */

    /* get the total number of pages to print */
    /* ... same code as above example */

    /***** print the pages in the background */
    XtAppAddWorkProc(app_context, PrintOnePageWP, n_pages);
}SEE ALSO&cdeman.XmPrintSetup;,
&cdeman.XmPrintShell;