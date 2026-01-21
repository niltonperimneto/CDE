# XmClipboardCopy
library call`XmClipboardCopy`A clipboard function that copies a data item to temporary storage for later copying to clipboardXmClipboardCopyclipboard functionsXmClipboardCopy#include <Xm/CutPaste.h>
int XmClipboardCopy (display, window, item_id, format_name,
        buffer, length, private_id, data_id)
        Display* display;
        Windowwindow;
        longitem_id;
        char* format_name;
        XtPointerbuffer;
        unsigned longlength;
        longprivate_id;
        long* data_id;
## DESCRIPTION


`XmClipboardCopy`copies a data item to temporary storage.
The data item is moved from temporary storage
to the clipboard data structure when a
call to`XmClipboardEndCopy`is made.
Additional calls to`XmClipboardCopy`before a call to`XmClipboardEndCopy`add
additional data item formats to the same data item or
append data to an existing format. Formats are described in theInter-Client Communication Conventions Manual(ICCCM) as targets.

`NOTE:`Do not call`XmClipboardCopy`before a call to`XmClipboardStartCopy`has been made. The latter function allocates
temporary storage required by`XmClipboardCopy`.

If the`buffer`argument is NULL, the data is considered
to be passed by name.
When data that
has been passed by name is later requested by another application, the
application that owns the data receives a callback with a request for
the data. The application that owns the data must then transfer the
data to the clipboard with the`XmClipboardCopyByName`function.
When a data item that was passed by name is deleted
from the clipboard, the application that owns the data receives a
callback stating that the data is no longer needed.

For information on the callback function, see the callback argument
description for`XmClipboardStartCopy`.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`item_id`** 

Specifies the number assigned to this data item.
This number was returned by a previous call to`XmClipboardStartCopy`.
* **`format_name`** 

Specifies the name of the format in which the data item
is stored on the clipboard. The format was known as target in the ICCCM.
* **`buffer`** 

Specifies the buffer from which the clipboard copies the data.
* **`length`** 

Specifies the
length, in bytes,
of the data being copied to the clipboard.
* **`private_id`** 

Specifies the private data that the application wants
to store with the data item.
* **`data_id`** 

Specifies an identifying number assigned to the data item that uniquely
identifies the data item and the format.
This argument is required only for data that is passed by name.

## RETURN


* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.
* **`XmClipboardFail`** 

The function failed because`XmClipboardStartCopy`was not called or
because the data item contains too many formats.

## RELATED


&cdeman.XmClipboardCopyByName;,
&cdeman.XmClipboardEndCopy;, and
&cdeman.XmClipboardStartCopy;.