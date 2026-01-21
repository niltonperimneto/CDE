# XmScrolledWindowSetAreas
library call`XmScrolledWindowSetAreas`A ScrolledWindow function that adds or changes a window work region and a horizontal or vertical ScrollBar widget to the ScrolledWindow widgetXmScrolledWindowSetAreasScrolledWindow functionsXmScrolledWindowSetAreas#include <Xm/ScrolledW.h>void`XmScrolledWindowSetAreas`WidgetwidgetWidgethorizontal_scrollbarWidgetvertical_scrollbarWidgetwork_region
## DESCRIPTION


`XmScrolledWindowSetAreas`adds or changes a window work region and
a horizontal or vertical ScrollBar widget to the ScrolledWindow widget for
the application. Each widget is optional and may be passed as NULL.
This function is obsolete and exists for compatibility with
other releases. Use the`XmNscrolledWindowChildType`resource
of`XmScrolledWindow`instead.

* **`widget`** 

Specifies the ScrolledWindow widget ID.
* **`horizontal_scrollbar`** 

Specifies the ScrollBar widget ID for the
horizontal ScrollBar to be associated
with the ScrolledWindow widget. Set this ID only after creating an instance
of the ScrolledWindow widget. The resource name associated with this
argument is`XmNhorizontalScrollBar`.
* **`vertical_scrollbar`** 

Specifies the ScrollBar widget ID for the
vertical ScrollBar to be associated
with the ScrolledWindow widget. Set this ID only after creating an instance
of the ScrolledWindow widget. The resource name associated with this
argument is`XmNverticalScrollBar`.
* **`work_region`** 

Specifies the widget ID for the work window to be associated with the
ScrolledWindow widget. Set this ID only after creating an instance of the
ScrolledWindow widget. The attribute name associated with this argument is`XmNworkWindow`.


For a complete definition of ScrolledWindow and its associated resources, see
&cdeman.XmScrolledWindow;.
## RELATED


&cdeman.XmScrolledWindow;.