# XmDropTransferStart
library call`XmDropTransferStart`A Drag and Drop function that initiates a
drop transferXmDropTransferStartDrag and Drop functionsXmDropTransferStart#include <Xm/DragDrop.h>Widget`XmDropTransferStart`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmDropTransferStart`initiates a drop transfer and uses the
specified argument list to initialize an`XmDropTransfer`object.
The DropTransfer object can be manipulated with`XtSetValues`and`XtGetValues`until the last call to the`XmNtransferProc`procedure is made. After that point, the result of using the
widget pointer is undefined. The DropTransfer object is freed
by the toolkit when a transfer is complete.

* **`widget`** 

Specifies the ID of the DragContext widget associated with the
transaction
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`)


For a complete definition of DropTransfer and its associated resources,
see &cdeman.XmDropTransfer;.
## RETURN


Returns the ID of the DropTransfer widget.
## RELATED


&cdeman.XmDragContext;,
&cdeman.XmDropTransfer;, and
&cdeman.XmDropTransferAdd;.