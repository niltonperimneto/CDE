# XmClipboardUndoCopy
library call`XmClipboardUndoCopy`A clipboard function that deletes the last item placed on the clipboardXmClipboardUndoCopyclipboard functionsXmClipboardUndoCopy#include <Xm/CutPaste.h>
int XmClipboardUndoCopy (display, window)
        Display* display;
        Windowwindow;
## DESCRIPTION


`XmClipboardUndoCopy`deletes the last item placed on the clipboard if the item
was placed there by an application with the passed`display`and`window`arguments. Any data item deleted from the clipboard by the
original call to`XmClipboardCopy`is restored. If the`display`or`window`IDs do not match the last copied item, no action is taken,
and this function has no effect.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each
clipboard function it calls.

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


&cdeman.XmClipboardLock; and &cdeman.XmClipboardStartCopy;.