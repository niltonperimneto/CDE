# XmDragCancel
library call`XmDragCancel`A Drag and Drop function that terminates a drag
transactionXmDragCancelDrag and Drop functionsXmDragCancel#include <Xm/DragDrop.h>void`XmDragCancel`Widgetdragcontext
## DESCRIPTION


`XmDragCancel`terminates a drag operation and cancels
any pending actions of the specified DragContext. This routine
can only be called by the initiator client.

* **`dragcontext`** 

Specifies the ID of the DragContext widget associated with the
drag and drop transaction to be terminated


For a complete definition of DragContext and its associated resources,
see &cdeman.XmDragContext;.
## RELATED


&cdeman.XmDragContext; and
&cdeman.XmDragStart;.