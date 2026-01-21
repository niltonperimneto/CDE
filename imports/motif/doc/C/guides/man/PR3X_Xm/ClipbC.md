# XmClipboardCopyByName
library call`XmClipboardCopyByName`A clipboard function that copies a data item passed by nameXmClipboardCopyByNameclipboard functionsXmClipboardCopyByName#include <Xm/CutPaste.h>
int XmClipboardCopyByName (display, window, data_id,
        buffer, length, private_id)
        Display* display;
        Windowwindow;
        longdata_id;
        XtPointerbuffer;
        unsigned longlength;
        longprivate_id;
## DESCRIPTION


`XmClipboardCopyByName`copies the actual data for a data item
that was previously passed by name to the clipboard. Data is
considered to be passed by name when a call to`XmClipboardCopy`is made with a NULL buffer parameter. Additional calls to this
function append new data to the existing data.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each
clipboard function it calls.
* **`data_id`** 

Specifies an identifying number assigned to the data item that uniquely
identifies the data item and the format. This number was assigned by`XmClipboardCopy`to the data item.
* **`buffer`** 

Specifies the buffer from which the clipboard copies the data.
* **`length`** 

Specifies the number of bytes in the data item.
* **`private_id`** 

Specifies the private data that the application wants to store with the
data item.

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


&cdeman.XmClipboardCopy;, &cdeman.XmClipboardLock;,
&cdeman.XmClipboardStartCopy;, and &cdeman.XmClipboardUnlock;.