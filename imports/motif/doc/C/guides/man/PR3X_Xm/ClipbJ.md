# XmClipboardLock
library call`XmClipboardLock`A clipboard function that locks the clipboardXmClipboardLockclipboard functionsXmClipboardLock#include <Xm/CutPaste.h>
int XmClipboardLock (display, window)
        Display* display;
        Windowwindow;
## DESCRIPTION


`XmClipboardLock`locks the clipboard from access by another
application until`XmClipboardUnlock`is called. All clipboard
functions lock and unlock the clipboard to prevent simultaneous
access. This function allows the application to keep the clipboard
data from changing between calls to`Inquire`and
other clipboard functions. The application does not need to
lock the clipboard between calls to`XmClipboardStartCopy`and`XmClipboardEndCopy`or to`XmClipboardStartRetrieve`and`XmClipboardEndRetrieve`.

If the clipboard is already locked by another application,`XmClipboardLock`returns an error status. Multiple calls to this
function by the same application increase the lock level.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
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


&cdeman.XmClipboardEndCopy;, &cdeman.XmClipboardEndRetrieve;,
&cdeman.XmClipboardStartCopy;, &cdeman.XmClipboardStartRetrieve;, and
&cdeman.XmClipboardUnlock;.