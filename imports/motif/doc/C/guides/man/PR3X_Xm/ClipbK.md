# XmClipboardRegisterFormat
library call`XmClipboardRegisterFormat`A clipboard function that registers a new formatXmClipboardRegisterFormatclipboard functionsXmClipboardRegisterFormat#include <Xm/CutPaste.h>
int XmClipboardRegisterFormat (display, format_name, format_length)
        Display* display;
        char* format_name;
        intformat_length;
## DESCRIPTION


`XmClipboardRegisterFormat`registers a new format. Each format
stored on the clipboard should have a length associated with it; this
length must be known to the clipboard routines. Formats are known as
targets in theInter-Client Communication Conventions Manual(ICCCM). All of the formats specified by
version 1.1 of the ICCCM
conventions are preregistered. Any other format that the application
wants to use must either be 8-bit
data or be registered via this routine.
Failure to
register the length of the data results in incompatible applications across
platforms having different byte-swapping orders.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`format_name`** 

Specifies the string name for the new format (target).
* **`format_length`** 

Specifies the format length in bits (8, 16, or 32).

## RETURN


* **`XmClipboardBadFormat`** 

The`format_name`must not be NULL, and the`format_length`must be 8, 16, or 32.
* **`XmClipboardSuccess`** 

The function was successful.
* **`XmClipboardLocked`** 

The function failed because the clipboard was locked by another
application. The application can continue to call the function again with
the same parameters until the lock goes away. This gives the application
the opportunity to ask if the user wants to keep trying or to give up
on the operation.
* **`XmClipboardFail`** 

The function failed because the specified format was already registered with a
different length from that specified now. If a specified format was already
registered with the same
length as that specified now,`XmClipboardSuccess`is returned.

## RELATED


&cdeman.XmClipboardStartCopy;.