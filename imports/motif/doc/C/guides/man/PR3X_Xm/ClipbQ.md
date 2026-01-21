# XmClipboardWithdrawFormat
library call`XmClipboardWithdrawFormat`A clipboard function that indicates that the application no longer wants to supply a data itemXmClipboardWithdrawFormatclipboard functionsXmClipboardWithdrawFormat#include <Xm/CutPaste.h>
int XmClipboardWithdrawFormat (display, window, data_id)
        Display* display;
        Windowwindow;
        longdata_id;
## DESCRIPTION


`XmClipboardWithdrawFormat`indicates that the application no longer
supplies a data item to the clipboard that the application had
previously passed by name.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that
relates the application window to the clipboard.
The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each
clipboard function it calls.
* **`data_id`** 

Specifies an identifying number assigned to the data item, that
uniquely identifies the data item and the format. This was assigned
to the item when it was originally passed by`XmClipboardCopy`.

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


&cdeman.XmClipboardCopy; and &cdeman.XmClipboardStartCopy;.