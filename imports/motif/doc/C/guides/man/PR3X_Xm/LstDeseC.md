# XmListDeselectPos
library call`XmListDeselectPos`A List function that deselects an item at a specified position in the listXmListDeselectPosList functionsXmListDeselectPos#include <Xm/List.h>void`XmListDeselectPos`Widgetwidgetintposition
## DESCRIPTION


`XmListDeselectPos`unhighlights the item at the specified position
and deletes it from the list of selected items.

* **`widget`** 

Specifies the ID of the List widget
* **`position`** 

Specifies the position of the item to be deselected.
A value of 1 indicates that the first item in the list is deselected; a
value of 2 indicates that the second item is deselected; and so on.
A value of 0 (zero) indicates that the last item in the list is deselected.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.