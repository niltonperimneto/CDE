# XmListDeleteItems
library call`XmListDeleteItems`A List function that deletes items from the listXmListDeleteItemsList functionsXmListDeleteItems#include <Xm/List.h>void`XmListDeleteItems`WidgetwidgetXmString *itemsintitem_count
## DESCRIPTION


`XmListDeleteItems`deletes the specified items from the list.
For each element of`items`, the first item in the list that matches
that element is deleted.
A warning message appears if any of the items do not exist.

* **`widget`** 

Specifies the ID of the List from whose list an item is deleted
* **`items`** 

Specifies a pointer to items to be deleted from the list
* **`item_count`** 

Specifies the number of elements in`items`This number must be nonnegative.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.