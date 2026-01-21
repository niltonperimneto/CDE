# XmClipboardInquireLength
library call`XmClipboardInquireLength`A clipboard function that returns the length of the stored dataXmClipboardInquireLengthclipboard functionsXmClipboardInquireLength#include <Xm/CutPaste.h>
int XmClipboardInquireLength (display, window, format_name, length)
        Display* display;
        Windowwindow;
        char* format_name;
        unsigned long* length;
## DESCRIPTION


`XmClipboardInquireLength`returns the length of the data stored
under a specified format name for the clipboard data item. If no data
is found for the specified format, or if there is no item on the
clipboard, this function returns a value of 0
(zero) in the`length`argument.

Any format passed by name is assumed to have`length`passed in
a call to`XmClipboardCopy`, even though the data has not yet been
transferred to the clipboard in that format.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`format_name`** 

Specifies the name of the format for the data item.
* **`length`** 

Specifies the length of the next data item in the specified format. This
argument equals 0 (zero) if no data is found for the specified format,
or if there is no item on the clipboard.

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


&cdeman.XmClipboardCopy; and &cdeman.XmClipboardStartCopy;.