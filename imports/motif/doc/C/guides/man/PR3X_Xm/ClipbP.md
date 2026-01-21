# XmClipboardUnlock
library call`XmClipboardUnlock`A clipboard function that unlocks the clipboardXmClipboardUnlockclipboard functionsXmClipboardUnlock#include <Xm/CutPaste.h>
int XmClipboardUnlock (display, window, remove_all_locks)
        Display* display;
        Windowwindow;
        Booleanremove_all_locks;
## DESCRIPTION


`XmClipboardUnlock`unlocks the clipboard,
enabling it to be accessed by other applications.

If multiple calls to`XmClipboardLock`have occurred, the same
number of calls to`XmClipboardUnlock`is necessary to unlock the
clipboard, unless`remove_all_locks`is set to True.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`remove_all_locks`** 

When True, indicates that all nested locks should be removed. When False,
indicates that only one level of lock should be removed.

## RETURN


* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardFail`** 

The function failed because the clipboard was not locked or was locked
by another application.

## RELATED


&cdeman.XmClipboardCancelCopy;, &cdeman.XmClipboardCopy;,
&cdeman.XmClipboardEndCopy;, &cdeman.XmClipboardEndRetrieve;,
&cdeman.XmClipboardInquireCount;, &cdeman.XmClipboardInquireFormat;,
&cdeman.XmClipboardInquireLength;, &cdeman.XmClipboardInquirePendingItems;,
&cdeman.XmClipboardLock;, &cdeman.XmClipboardRegisterFormat;,
&cdeman.XmClipboardRetrieve;, &cdeman.XmClipboardStartCopy;,
&cdeman.XmClipboardStartRetrieve;, &cdeman.XmClipboardUndoCopy;,
and &cdeman.XmClipboardWithdrawFormat;.