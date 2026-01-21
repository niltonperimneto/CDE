# XmClipboardCancelCopy
library call`XmClipboardCancelCopy`A clipboard function that cancels a copy to the clipboardXmClipboardCancelCopyclipboard functionsXmClipboardCancelCopy#include <Xm/CutPaste.h>
int XmClipboardCancelCopy (display, window, item_id)
        Display* display;
        Windowwindow;
        longitem_id;
## DESCRIPTION


`XmClipboardCancelCopy`cancels the copy to clipboard that is in progress and
frees up temporary storage.
When a copy is to be performed,`XmClipboardStartCopy`allocates
temporary storage for the clipboard data.`XmClipboardCopy`copies
the appropriate data into the the temporary storage.`XmClipboardEndCopy`copies the data to the clipboard structure and frees up the temporary
storage structures.
If`XmClipboardCancelCopy`is called, the`XmClipboardEndCopy`function does not have to be called. A call to`XmClipboardCancelCopy`is valid only after a call to`XmClipboardStartCopy`.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies a widget's window ID that
relates the application window to the clipboard.
The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`item_id`** 

Specifies the number assigned to this data item. This number was returned
by a previous call to`XmClipboardStartCopy`.

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


&cdeman.XmClipboardCopy;,
&cdeman.XmClipboardEndCopy;, and &cdeman.XmClipboardStartCopy;.