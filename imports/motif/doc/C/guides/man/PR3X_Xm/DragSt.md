# XmDragStart
library call`XmDragStart`A Drag and Drop function that initiates a drag and
drop transactionXmDragStartDrag and Drop functionsXmDragStart#include <Xm/DragDrop.h>Widget`XmDragStart`WidgetwidgetXEvent *eventArgListarglistCardinalargcount
## DESCRIPTION


`XmDragStart`initiates a drag operation. This routine
returns the DragContext widget that it initializes for the
associated drag transaction. The toolkit is responsible for
freeing the DragContext when the drag and drop transaction
is complete.

* **`widget`** 

Specifies the ID of the smallest widget and/or gadget that encloses
the source elements selected for a drag operation.
* **`event`** 

Specifies the`XEvent`that triggered the drag operation. This
event must be a ButtonPress event.
* **`arglist`** 

Specifies the argument list. Any`XmDragContext`resources not
specified in the argument list are obtained from the resource database
or are set to their default values.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`)


For a complete definition of DragContext and its associated resources,
see &cdeman.XmDragContext;.
## RETURN


Returns the ID of the DragContext widget that controls this drag
and drop transaction.
Returns NULL if the drag cannot be initiated.
## RELATED


&cdeman.XmDragCancel; and
&cdeman.XmDragContext;.