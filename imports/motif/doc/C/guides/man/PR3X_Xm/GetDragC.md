# XmGetDragContext
library call`XmGetDragContext`A Drag and Drop function that retrieves the
DragContext widget ID associated with a timestampXmGetDragContextDrag and Drop functionsXmGetDragContext#include <Xm/DragC.h>Widget`XmGetDragContext`WidgetrefwidgetTimetimestamp
## DESCRIPTION


`XmGetDragContext`returns the widget ID of the active
DragContext associated with a given display and timestamp. A timestamp
uniquely identifies which DragContext is active when more than
one drag and drop transaction has been initiated on a display. If
the specified timestamp matches a timestamp processed between
the start and finish of a single drag and drop transaction, the
function returns the corresponding DragContext ID.

* **`refwidget`** 

Specifies the ID of the widget that the routine uses to identify
the intended display. The function returns the ID of the
DragContext associated with the display value passed by this widget.
* **`timestamp`** 

Specifies a timestamp.


For a complete definition of DragContext and its associated resources,
see &cdeman.XmDragContext;.
## RETURN


Returns the ID of the DragContext widget that is active for the
specified timestamp. Otherwise, returns NULL if no active
DragContext is found.
## RELATED


&cdeman.XmDragContext;.