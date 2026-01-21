# XmListSetKbdItemPos
library call`XmListSetKbdItemPos`A List function that sets the
location cursor at a specified positionXmListSetKbdItemPosList functionsXmListSetKbdItemPos#include <Xm/List.h>Boolean`XmListSetKbdItemPos`Widgetwidgetintposition
## DESCRIPTION


`XmListSetKbdItemPos`sets the location cursor at the
item specified by`position`. This function does not
determine if the item at the specified position is
selected or not.

* **`widget`** 

Specifies the ID of the List widget.
* **`position`** 

Specifies the position of the item at which the location
cursor is set. A value of 1 indicates the first item in
the list; a value of 2 indicates the second item; and so
on. A value of 0 (zero) sets the location cursor at the last item
in the list.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns False if no item exists at the specified position or if
the list is empty; otherwise, returns True.
## RELATED


&cdeman.XmList;.