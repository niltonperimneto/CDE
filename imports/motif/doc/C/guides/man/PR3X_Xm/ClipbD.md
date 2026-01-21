# XmClipboardEndCopy
library call`XmClipboardEndCopy`A clipboard function that completes the
copying of data to the clipboardXmClipboardEndCopyclipboard functionsXmClipboardEndCopy#include <Xm/CutPaste.h>
int XmClipboardEndCopy (display, window, item_id)
        Display* display;
        Windowwindow;
        longitem_id;
## DESCRIPTION


`XmClipboardEndCopy`locks the clipboard from access by other applications,
places data in the clipboard data structure, and unlocks the clipboard.
Data items copied to the clipboard by`XmClipboardCopy`are not actually
entered in the clipboard data structure until the call to`XmClipboardEndCopy`.

This function also frees up temporary storage that was allocated by`XmClipboardStartCopy`, which must be called before`XmClipboardEndCopy`. The latter function should not be called if`XmClipboardCancelCopy`has been called.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each
clipboard function it calls.
* **`item_id`** 

Specifies the number assigned to this data item, which was returned
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

The function failed because`XmClipboardStartCopy`was not called.

## RELATED


&cdeman.XmClipboardCancelCopy;,
&cdeman.XmClipboardCopy; and &cdeman.XmClipboardStartCopy;.