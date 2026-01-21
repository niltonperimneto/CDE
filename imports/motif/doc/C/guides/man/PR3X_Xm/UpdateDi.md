# XmUpdateDisplay
library call`XmUpdateDisplay`A function that processes all pending exposure events immediatelyXmUpdateDisplayvoid XmUpdateDisplay (widget)
        Widgetwidget;
## DESCRIPTION


`XmUpdateDisplay`provides the application with a mechanism for forcing all
pending exposure events to be removed from the input queue and processed immediately.
When a user selects a button within a menu pane, the menu panes are
unposted and then any activation callbacks registered by the application
are invoked. If one of the callbacks performs a time-consuming action,
the portion of the application window that was covered by the menu panes
will not have been redrawn; normal exposure processing does not occur until
all of the callbacks have been invoked. If the
application writer suspects that a callback
will take a long time, then the callback may choose to invoke`XmUpdateDisplay`before starting its time-consuming operation.
This function is also useful any time a transient window, such as a dialog box, is unposted;
callbacks are invoked before normal exposure processing can occur.

* **`widget`** 

Specifies any widget or gadget.
