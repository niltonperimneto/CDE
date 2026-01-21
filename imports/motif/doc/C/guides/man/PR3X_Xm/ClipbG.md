# XmClipboardInquireFormat
library call`XmClipboardInquireFormat`A clipboard function that returns a specified format nameXmClipboardInquireFormatclipboard functionsXmClipboardInquireFormat#include <Xm/CutPaste.h>
int XmClipboardInquireFormat (display, window, index, format_name_buf,
        buffer_len, copied_len)
        Display* display;
        Windowwindow;
        intindex;
        XtPointerformat_name_buf;
        unsigned longbuffer_len;
        unsigned long* copied_len;
## DESCRIPTION


`XmClipboardInquireFormat`returns a specified format name for the
data item in the clipboard. If the name must be truncated, the
function returns a warning status.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`index`** 

Specifies which of the ordered format names to obtain. If this index
is greater than the number of formats for the data item,
this function returns a 0 (zero) in the`copied_len`argument.
* **`format_name_buf`** 

Specifies the buffer that receives the format name.
* **`buffer_len`** 

Specifies the number of bytes in the format name buffer.
* **`copied_len`** 

Specifies the number of bytes in the
data item copied to the buffer.
If this
argument equals 0 (zero), there is no`n`th format for the data item.

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
format requested.
This could occur because the clipboard is empty; there is data
on the clipboard, but not in the requested format; or the data in
the requested format was passed by name and is no longer available.

## RELATED


&cdeman.XmClipboardStartCopy;.