# XmListItemPos
library call`XmListItemPos`A List function that returns the position of an item in the listXmListItemPosList functionsXmListItemPos#include <Xm/List.h>int`XmListItemPos`WidgetwidgetXmStringitem
## DESCRIPTION


`XmListItemPos`returns the position of the first
instance of the specified item in a list.

* **`widget`** 

Specifies the ID of the List widget
* **`item`** 

Specifies the item whose position is returned


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns the position in the list of the first instance of the specified
item.
The position of the first item in the list is 1; the position of the
second item is 2; and so on.
This function returns 0 (zero) if the item is not found.
## RELATED


&cdeman.XmList;.