# XmObjectAtPoint
library call`XmObjectAtPoint`A toolkit function that determines which child intersects or comes closest to a specified pointXmObjectAtPoint#include <Xm/Xm.h>Widget`XmObjectAtPoint`WidgetwidgetPositionxPositiony
## DESCRIPTION


`XmObjectAtPoint`searches the child list of the specified
manager`widget`and returns the child most closely associated
with the specifiedx,ycoordinate pair.

For the typical Motif manager`widget`,`XmObjectAtPoint`uses the following rules to determine the returned object:

If one child intersectsx,y,`XmObjectAtPoint`returns the widget ID of that child.

If more than one child intersectsx,y,`XmObjectAtPoint`returns the widget ID of the visible child.

If no child intersectsx,y,`XmObjectAtPoint`returns NULL.

The preceding rules are only general. In fact, each manager`widget`is free to define "most closely associated"
as it desires.
For example, if no child intersectsx,y, a manager
might return the child closest tox,y.

* **`widget`** 

Specifies a manager widget.
* **x** 

Specifies the x-coordinate about which you are seeking child
information. The x-coordinate must be specified in pixels,
relative to the left side ofmanager.
* **y** 

Specifies the y-coordinate about which you are seeking child
information. The y-coordinate must be specified in pixels,
relative to the top side ofmanager.

## RETURN


Returns the child ofmanagermost closely associated
withx,y.
If none of its children are sufficiently associated withx,y,
returns NULL.
## RELATED


&cdeman.XmManager;.