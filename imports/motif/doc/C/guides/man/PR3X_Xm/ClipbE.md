# XmClipboardEndRetrieve
library call`XmClipboardEndRetrieve`A clipboard function that completes
retrieval of data from the clipboardXmClipboardEndRetrieveclipboard functionsXmClipboardEndRetrieve#include <Xm/CutPaste.h>
int XmClipboardEndRetrieve (display, window)
        Display* display;
        Windowwindow;
## DESCRIPTION


`XmClipboardEndRetrieve`suspends copying data incrementally from
the clipboard. It tells the clipboard routines that the application
is through copying an item from the clipboard. Until this function is
called, data items can be retrieved incrementally from the clipboard
with`XmClipboardRetrieve`.
The act of copying data is started with the`XmClipboardStartRetrieve`function.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained with`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.

## RETURN


* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.

## RELATED


&cdeman.XmClipboardRetrieve;, &cdeman.XmClipboardStartCopy;, and
&cdeman.XmClipboardStartRetrieve;.