# XmListSetBottomPos
library call`XmListSetBottomPos`A List function that makes a specified item the last visible item in the listXmListSetBottomPosList functionsXmListSetBottomPos#include <Xm/List.h>void`XmListSetBottomPos`Widgetwidgetintposition
## DESCRIPTION


`XmListSetBottomPos`makes the item at the specified position the
last visible item in the List.

* **`widget`** 

Specifies the ID of the List widget.
* **`position`** 

Specifies the position of the item to be made the last visible item in
the list.
A value of 1 indicates that the first item in the list is the last
visible item; a value of 2 indicates that the second item is the last
visible item; and so on.
A value of 0 (zero) indicates that the last item in the list is the last
visible item.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.