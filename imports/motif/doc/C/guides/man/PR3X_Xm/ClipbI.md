# XmClipboardInquirePendingItems
library call`XmClipboardInquirePendingItems`A clipboard function that returns a list of data ID/private ID pairsXmClipboardInquirePending\\%Itemsclipboard functionsXmClipboardInquirePending\\%Items#include <Xm/CutPaste.h>
int XmClipboardInquirePendingItems (display, window, format_name, item_list, count)
        Display* display;
        Windowwindow;
        char* format_name;
        XmClipboardPendingList* item_list;
        unsigned long* count;
## DESCRIPTION


`XmClipboardInquirePendingItems`returns a list of
data ID/private ID pairs
for the specified format name. A data item is considered pending if the
application originally passed it by name, the application has not yet
copied the data, and the item has not been deleted from the clipboard.
The application is responsible for freeing the memory provided by this
function to store the list.
To free the memory, call`XtFree`.

This function is used by an application when exiting, to determine if the
data that is passed by name should be sent to the clipboard.

* **`display`** 

Specifies a pointer to theDisplaystructure that was returned in a
previous call to`XOpenDisplay`or`XtDisplay`.
* **`window`** 

Specifies the window ID of a widget that relates the application window to the
clipboard. The widget's window ID can be obtained through`XtWindow`.
The same application instance should pass the same window ID to each of the
clipboard functions that it calls.
* **`format_name`** 

Specifies a string that contains the name of the format for which the list
of data ID/private ID pairs is to be obtained.
* **`item_list`** 

Specifies the address of the array of data ID/private ID pairs for the
specified format name. This argument is a typeXmClipboardPendingList.
The application is
responsible for freeing the memory provided by this function
for storing the list.
* **`count`** 

Specifies the number of items returned in the list. If there is no data for
the specified format name, or if there is no item on the clipboard, this
argument equals 0 (zero).

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


&cdeman.XmClipboardStartCopy;.