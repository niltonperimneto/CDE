# XmListAddItems
library call`XmListAddItems`A List function that adds items to the listXmListAddItemsList functionsXmListAddItems#include <Xm/List.h>void`XmListAddItems`WidgetwidgetXmString *itemsintitem_countintposition
## DESCRIPTION


`XmListAddItems`adds the specified items to the list at the given
position.
The first`item_count`items of the`items`array are added to
the list.
When the items are inserted into the list, they are compared with the
current`XmNselectedItems`list.
If any of the new items matches an item on the selected list, it
appears selected.

* **`widget`** 

Specifies the ID of the List to which an item is added.
* **`items`** 

Specifies a pointer to the items to be added to the list.
* **`item_count`** 

Specifies the number of items in`items`.
This number must be nonnegative.
* **`position`** 

Specifies the position of the first new item in the list.
A value of 1 makes the first new item the first item in the list; a
value of 2 makes it the second item; and so on.
A value of 0 (zero) makes the first new item follow the last item in the list.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.