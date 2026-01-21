# XmClipboardStartRetrieve
library call`XmClipboardStartRetrieve`A clipboard function that prepares
to retrieve data from the clipboardXmClipboardStartRetrieveclipboard functionsXmClipboardStartRetrieve#include <Xm/CutPaste.h>
int XmClipboardStartRetrieve (display, window, timestamp)
        Display* display;
        Windowwindow;
        Timetimestamp;
## DESCRIPTION


`XmClipboardStartRetrieve`tells the clipboard routines that the application is
ready to start copying an item from the clipboard.
The clipboard is locked by this routine and stays locked until`XmClipboardEndRetrieve`is called. Between a call to`XmClipboardStartRetrieve`and a call to`XmClipboardEndRetrieve`, multiple calls to`XmClipboardRetrieve`with the same format name result in data
being incrementally copied from the clipboard until the data in that
format has all been retrieved.

A return value of`XmClipboardTruncate`from calls to`XmClipboardRetrieve`indicates that more data remains to be
copied in the given format.
It is recommended that any calls to the`Inquire`functions that
the application needs to make to complete the copy from the clipboard
be made between the call to`XmClipboardStartRetrieve`and the
first call to`XmClipboardRetrieve`. This way, the application
does not need to call`XmClipboardLock`and`XmClipboardUnlock`.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`timestamp`** 

Specifies the time of the event that triggered the copy.
A valid timestamp must be supplied; it is not sufficient to use`CurrentTime`.

## RETURN


* **`XmClipboardSuccess`** 

The function is successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.

## RELATED


&cdeman.XmClipboardEndRetrieve;,
&cdeman.XmClipboardInquireCount;, &cdeman.XmClipboardInquireFormat;,
&cdeman.XmClipboardInquireLength;, &cdeman.XmClipboardInquirePendingItems;,
&cdeman.XmClipboardLock;,
&cdeman.XmClipboardRetrieve;, &cdeman.XmClipboardStartCopy;, and
&cdeman.XmClipboardUnlock;.