# XmIsTraversable
library call`XmIsTraversable`A function that identifies whether a widget can
be traversedXmIsTraversable#include <Xm/Xm.h>Boolean`XmIsTraversable`Widgetwidget
## DESCRIPTION


`XmIsTraversable`determines whether the specified widget is
eligible to receive focus through keyboard traversal.
In general, a widget is eligible to receive focus when all of the
following conditions are true:

The widget and its ancestors are not being destroyed, are sensitive, and
have a value of True for`XmNtraversalOn`.

The widget and its ancestors are realized, managed, and (except for
gadgets) mapped.
If an application unmaps a`widget`that has its`XmNmappedWhenManaged`resource set to True, the return value
is undefined.

Some part of the widget's rectangular area is unobscured by the widget's
ancestors, or some part of the widget's rectangular area is inside the
work window (but possibly outside the clip window) of a ScrolledWindow
whose`XmNscrollingPolicy`is`XmAUTOMATIC`and whose`XmNtraverseObscuredCallback`is not NULL.

Some widgets may not be eligible to receive focus even if they meet all
these conditions.
For example, most managers cannot receive focus through keyboard traversal.
Some widgets may be eligible to receive focus under particular
conditions.
For example, a DrawingArea is eligible to receive focus if it meets the
conditions above and has no child whose`XmNtraversalOn`resource is
True.

Note that when all widgets in a shell hierarchy have been made
untraversable, they are considered to have lost focus. When a
widget in this hierarchy is made traversable again, it regains focus.

`XmIsTraversable`may return unexpected results when`widget`or its ancestors are overlapped by their siblings.

* **`widget`** 

Specifies the ID of the widget

## RETURN


Returns True if the widget is eligible to receive focus through keyboard
traversal; otherwise, returns False.
## RELATED


&cdeman.XmGetVisibility; and &cdeman.XmProcessTraversal;.