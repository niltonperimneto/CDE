# XmGetVisibility
library call`XmGetVisibility`A function that determines if a widget is
visibleXmGetVisibility#include <Xm/Xm.h>XmVisibility`XmGetVisibility`Widgetwidget
## DESCRIPTION


`XmGetVisibility`returns the visibility state of the specified
widget.
It checks to see if some part of the widget's rectangular
area is unobscured
by the widget's ancestors, or some part of the widget's rectangular
area is inside the work window (but possibly outside the clip window)
of a ScrolledWindow whose`XmNscrollingPolicy`is`XmAUTOMATIC`and whose`XmNtraverseObscuredCallback`is not
NULL.

`XmGetVisibility`does not check to see if`widget`is obscured by
its siblings or by siblings of its ancestors. Consequently,`XmGetVisibility`returns`XmVISIBILITY_UNOBSCURED`for widgets which are completely or partially
covered by one or more siblings of`widget`by one or more siblings of ancestors
of`widget`.

When a widget which is unrealized is being queried, it is indicated
that the widget is fully obscured.
If an application unmaps a`widget`that has its`XmNmappedWhenManaged`resource set to True, the return value
is undefined.
When a widget which is unmanaged is being queried, it is
indicated that the widget is fully obscured.

* **`widget`** 

Specifies the ID of the widget

## RETURN


Returns one of the following values:

* **`XmVISIBILITY_UNOBSCURED`** 

Indicates that the widget is mapped, not obscured, and is completely
visible on the screen.
* **`XmVISIBILITY_PARTIALLY_OBSCURED`** 

Indicates that the widget is mapped, and is not completely
visible on the screen (partially obscured).
* **`XmVISIBILITY_FULLY_OBSCURED`** 

Indicates that the widget is not at all visible on the screen.

## RELATED


&cdeman.XmIsTraversable;,
&cdeman.XmManager;,
and &cdeman.XmProcessTraversal;.