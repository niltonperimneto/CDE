# XmListSetPos
library call`XmListSetPos`A List function that makes the item at the given position the first visible position in the listXmListSetPosList functionsXmListSetPos#include <Xm/List.h>void`XmListSetPos`Widgetwidgetintposition
## DESCRIPTION


`XmListSetPos`makes the item at the given position the first
visible position in the list.

* **`widget`** 

Specifies the ID of the List widget.
* **`position`** 

Specifies the position of the item to be made the first visible item in
the list.
A value of 1 indicates that the first item in the list is the first
visible item; a value of 2 indicates that the second item is the first
visible item; and so on.
A value of 0 (zero) indicates that the last item in the list is the first
visible item.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.