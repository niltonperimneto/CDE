# XmWidgetGetDisplayRect
library call`XmWidgetGetDisplayRect`Retrieves display rectangle
information for a widgetXmWidgetGetDisplayRect#include <Xm/Xm.h>Boolean`XmWidgetGetDisplayRect`WidgetwidgetXRectangle *displayrect
## DESCRIPTION


`XmWidgetGetDisplayRect`returns the width, height and
the x and y-coordinates of the upper left corner of the display
rectangle of the specified widget. The display rectangle
is the smallest rectangle that encloses either a string
or a pixmap.

If the widget contains a string, the return values specify
the x and y-coordinates of the upper left corner of
the display rectangle relative to the origin of the widget
and the width and height in pixels.

In the case of a pixmap, the return values
specify the x and y-coordinates of the upper left corner of
the pixmap relative to the origin, and the width
and height of the pixmap in pixels.

* **`widget`** 

Specifies the widget ID
* **`displayrect`** 

Specifies a pointer to an XRectangle structure in which the
x and y-coordinates, width and height of the display rectangle
are returned

## RETURN


Returns True if the specified widget has an associated
display rectangle; otherwise, returns False.
## RELATED


&cdeman.XmWidgetGetBaselines;.