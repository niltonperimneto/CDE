# XmListDeleteItemsPos
library call`XmListDeleteItemsPos`A List function that deletes items from the list starting at the given positionXmListDeleteItemsPosList functionsXmListDeleteItemsPos#include <Xm/List.h>void`XmListDeleteItemsPos`Widgetwidgetintitem_countintposition
## DESCRIPTION


`XmListDeleteItemsPos`deletes the specified number of items from
the list starting at the specified position.

* **`widget`** 

Specifies the ID of the List from whose list an item is deleted.
* **`item_count`** 

Specifies the number of items to be deleted.
This number must be nonnegative.
* **`position`** 

Specifies the position in the list of the first item to be deleted.
A value of 1 indicates that the first deleted item is the first item in
the list; a value of 2 indicates that it is the second item; and so on.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.