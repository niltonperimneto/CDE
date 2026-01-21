# XmDropSiteQueryStackingOrder
library call`XmDropSiteQueryStackingOrder`A Drag and Drop function that
returns the parent, a list of children, and the number of children
for a specified widgetXmDropSiteQueryStacking\\%OrderDrag and Drop functionsXmDropSiteQueryStacking\\%Order#include <Xm/DragDrop.h>Status`XmDropSiteQueryStackingOrder`WidgetwidgetWidget *parent_returnWidget **child_returnsCardinal *num_child_returns
## DESCRIPTION


`XmDropSiteQueryStackingOrder`obtains the parent, a list of
children registered as drop sites, and the number of children registered
as drop sites for a given widget. The children are listed in current
stacking order, from bottom-most (first child) to the top-most (last
child).This function allocates memory for the returned data that
must be freed by calling`XtFree`.

* **`widget`** 

Specifies the widget ID. For this widget, you obtain the list
of its children, its parent, and the number of children.
* **`parent_return`** 

Returns the widget ID of the drop site parent of the
specified widget.
* **`child_returns`** 

Returns a pointer to the list of drop site children associated with
the specified widget.
The function allocates memory to hold the list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XtFree`.
* **`num_child_returns`** 

Returns the number of drop site children for the specified widget.


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RETURN


Returns 0 (zero) if the routine fails; returns a nonzero
value if it succeeds.
## RELATED


&cdeman.XmDropSite; and
&cdeman.XmDropSiteConfigureStackingOrder;.