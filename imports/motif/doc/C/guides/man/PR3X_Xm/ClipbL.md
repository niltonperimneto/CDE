# XmClipboardRetrieve
library call`XmClipboardRetrieve`A clipboard function that retrieves a data item from the clipboardXmClipboardRetrieveclipboard functionsXmClipboardRetrieve#include <Xm/CutPaste.h>
int XmClipboardRetrieve (display, window, format_name,
        buffer, length, num_bytes, private_id)
        Display* display;
        Windowwindow;
        char* format_name;
        XtPointerbuffer;
        unsigned longlength;
        unsigned long* num_bytes;
        long* private_id;
## DESCRIPTION


`XmClipboardRetrieve`retrieves the current data item from clipboard
storage. It returns a warning if the clipboard is locked, if there is
no data on the clipboard, or if the data needs to be truncated because the
buffer length is too short.

Between a call
to`XmClipboardStartRetrieve`and a call to`XmClipboardEndRetrieve`,
multiple calls to`XmClipboardRetrieve`with the same format name result
in data being incrementally copied from the clipboard until the data in that
format has all been copied.

The return value`XmClipboardTruncate`from calls to`XmClipboardRetrieve`indicates that more data remains to be copied in the
given format.
It is recommended that any calls to the`Inquire`functions that
the application needs to make to effect the copy from the clipboard be
made between the call to`XmClipboardStartRetrieve`and the first
call to`XmClipboardRetrieve`. This way, the application does not
need to call`XmClipboardLock`and`XmClipboardUnlock`.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`format_name`** 

Specifies the name of a format in which the data
is stored on the clipboard.
* **`buffer`** 

Specifies the buffer to which the application wants the
clipboard to copy the data.
The function allocates space to hold the data returned into the buffer.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XtFree`.
* **`length`** 

Specifies the length of the application buffer.
* **`num_bytes`** 

Specifies the number of bytes of data copied into the application
buffer.
* **`private_id`** 

Specifies the private data stored with the data item by the
application that placed the data item on the clipboard. If the
application did not store private data with the data item, this
argument returns 0 (zero).

## RETURN


* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.
* **`XmClipboardTruncate`** 

The data returned is truncated because the user did not provide a buffer
large enough to hold the data.
* **`XmClipboardNoData`** 

The function could not find data on the clipboard corresponding to the
format requested. This could occur because the clipboard is empty;
there is data on the clipboard but not in the requested format; or the
data in the requested format was passed by name and is no longer
available.

## RELATED


&cdeman.XmClipboardEndRetrieve;, &cdeman.XmClipboardLock;,
&cdeman.XmClipboardStartCopy;, &cdeman.XmClipboardStartRetrieve;,
and &cdeman.XmClipboardUnlock;.