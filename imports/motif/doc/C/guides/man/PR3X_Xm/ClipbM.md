# XmClipboardStartCopy
library call`XmClipboardStartCopy`A clipboard function that sets up a storage and data structureXmClipboardStartCopyclipboard functionsXmClipboardStartCopy#include <Xm/CutPaste.h>
int XmClipboardStartCopy (display, window, clip_label,
        timestamp, widget, callback, item_id)
        Display* display;
        Windowwindow;
        XmStringclip_label;
        Timetimestamp;
        Widgetwidget;
        XmCutPasteProccallback;
        long* item_id;
## DESCRIPTION


`XmClipboardStartCopy`sets up storage and data structures to receive clipboard data.
An application calls this function during a cut or copy operation.
The data item that these structures receive then becomes
the next data item in the clipboard.

Copying a large piece of data to the clipboard can take a long time.
It is possible that, once the data is copied, no application will ever
request that data. The Motif Toolkit provides a mechanism so that an
application does not need to actually pass data to the clipboard until
the data has been requested by some application.

Instead, the application passes format and length information in`XmClipboardCopy`to the clipboard functions, along with a widget
ID and a callback function address that is passed in`XmClipboardStartCopy`. The widget ID is necessary for
communications between the clipboard functions in the application that
owns the data and the clipboard functions in the application that
requests the data.

The callback functions are responsible for copying the actual data to
the clipboard through`XmClipboardCopyByName`. The callback
function is also called if the data item is removed from the clipboard
and the actual data is no longer needed.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`clip_label`** 

Specifies the label to be associated with the data item.
This argument
is used to identify the data item, as in a clipboard viewer.
An example of a label is the name of the application that places the
data in the clipboard.
* **`timestamp`** 

Specifies the time of the event that triggered the copy.
A valid timestamp must be supplied; it is not sufficient to use`CurrentTime`.
* **`widget`** 

Specifies the ID of the widget that receives messages requesting data
previously passed by name. This argument must be present in order to
pass data by name. Any valid widget ID in your application can be
used for this purpose and all the message handling is taken care of by
the cut and paste functions.
* **`callback`** 

Specifies the address of the callback function that is called when the
clipboard needs data that was originally passed by name. This is also
the callback to receive the`delete`message for items that were
originally passed by name. This argument must be present in order to
pass data by name.
* **`item_id`** 

Specifies the number assigned to this data item.
The application uses this number in calls to`XmClipboardCopy`,`XmClipboardEndCopy`, and`XmClipboardCancelCopy`.


For more information on passing data by name, see
&cdeman.XmClipboardCopy; and &cdeman.XmClipboardCopyByName;.

The`widget`and`callback`arguments must be present in order to
pass data by name. The callback format is as follows:void (*callback)(widget, data_id, private, reason)
        Widgetwidget;
        long    *data_id;
        long    *private;
        int     *reason;

* **`widget`** 

Specifies the ID of the widget passed to this function.
* **`data_id`** 

Specifies the identifying number returned by`XmClipboardCopy`, which identifies the pass-by-name data.
* **`private`** 

Specifies the private information passed to`XmClipboardCopy`.
* **`reason`** 

Specifies the reason.`XmCR_CLIPBOARD_DATA_DELETE`or`XmCR_CLIPBOARD_DATA_REQUEST`are the possible values.

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


&cdeman.XmClipboardCancelCopy;, &cdeman.XmClipboardCopy;,
&cdeman.XmClipboardCopyByName;,
&cdeman.XmClipboardEndCopy;, &cdeman.XmClipboardEndRetrieve;,
&cdeman.XmClipboardInquireCount;, &cdeman.XmClipboardInquireFormat;,
&cdeman.XmClipboardInquireLength;, &cdeman.XmClipboardInquirePendingItems;,
&cdeman.XmClipboardLock;, &cdeman.XmClipboardRegisterFormat;,
&cdeman.XmClipboardRetrieve;,
&cdeman.XmClipboardStartRetrieve;, &cdeman.XmClipboardUndoCopy;,
&cdeman.XmClipboardUnlock;, and &cdeman.XmClipboardWithdrawFormat;.