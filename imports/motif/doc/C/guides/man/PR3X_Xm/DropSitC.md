# XmDropSiteConfigureStackingOrder
library call`XmDropSiteConfigureStackingOrder`A Drag and Drop function that
reorders a stack of widgets that are registered drop sitesXmDropSiteConfigureStack\\%ingOrderDrag and Drop functionsXmDropSiteConfigureStack\\%ingOrder#include <Xm/DragDrop.h>void`XmDropSiteConfigureStackingOrder`WidgetwidgetWidgetsiblingCardinalstack_mode
## DESCRIPTION


`XmDropSiteConfigureStackingOrder`changes the stacking
order of the drop site specified by`widget`. The stacking
order controls the manner in which drag-under effects are
clipped by overlapping siblings, regardless of whether they are
active. The stack mode is relative either to the entire stack,
or to another drop site within the stack. The stack order can
be modified only if the drop
sites are siblings in both the widget and drop site hierarchy, and
the widget parent of the drop sites is registered
as a composite drop site.

* **`widget`** 

Specifies the drop site to be restacked.
* **`sibling`** 

Specifies a sibling drop site for stacking operations. If specified,
then`widget`is restacked relative to this drop site's stack position.
* **`stack_mode`** 

Specifies the new stack position for the specified widget.
The values are`XmABOVE`and`XmBELOW`. If a sibling is specified,
then`widget`is restacked as follows:

* **`XmABOVE`** 

The widget is placed just above the sibling.
* **`XmBELOW`** 

The widget is placed just below the sibling.


If the`sibling`parameter is not specified, then`widget`is restacked as follows:

* **`XmABOVE`** 

The widget is placed at the top of the stack.
* **`XmBELOW`** 

The widget is placed at the bottom of the stack.



For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDropSite;,
&cdeman.XmDropSiteRetrieve;, and
&cdeman.XmDropSiteQueryStackingOrder;.