# XmCreateDragIcon
library call`XmCreateDragIcon`A Drag and Drop function that creates a DragIcon widgetXmCreateDragIconDrag and Drop functionsXmCreateDragIconcreation functionsXmCreateDragIcon#include <Xm/DragIcon.h>Widget`XmCreateDragIcon`WidgetwidgetStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateDragIcon`creates a DragIcon and returns the associated widget
ID.

* **`widget`** 

Specifies the ID of the widget that the function uses to access
default values for visual attributes of the DragIcon. This widget
may be different than the actual parent of the DragIcon.
* **`name`** 

Specifies the name of the DragIcon widget.
* **`arglist`** 

Specifies the argument list.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`).


For a complete definition of DragIcon and its associated resources,
see &cdeman.XmDragIcon;.
## RETURN


The function creates a DragIcon and returns the associated
widget ID.
## RELATED


&cdeman.XmDragContext;,
&cdeman.XmDragIcon;, and
&cdeman.XmScreen;.