# XmClipboardInquireCount
library call`XmClipboardInquireCount`A clipboard function that returns the number of data item formatsXmClipboardInquireCountclipboard functionsXmClipboardInquireCount#include <Xm/CutPaste.h>
int XmClipboardInquireCount (display, window, count,
        max_format_name_length)
        Display* display;
        Windowwindow;
        int* count;
        unsigned long* max_format_name_length;
## DESCRIPTION


`XmClipboardInquireCount`returns the number of data item formats available
for the data item in the clipboard. This function also returns the
maximum name-length for all formats in which the data item is
stored.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`count`** 

Returns the number of data item formats available for the data item in
the clipboard. If no formats are available, this argument equals 0
(zero). The count includes the formats that were passed by name.
* **`max_format_name_length`** 

Specifies the maximum length of all format names for the data item in
the clipboard.

## RETURN


* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.
* **`XmClipboardNoData`** 

The function could not find data on the clipboard corresponding to the
format requested. This could occur because the clipboard is empty;
there is data on the clipboard, but not in the requested format; or
the data in the requested format was passed by name and is no longer
available.

## RELATED


&cdeman.XmClipboardStartCopy;.