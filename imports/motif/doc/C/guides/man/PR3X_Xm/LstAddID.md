# XmListAddItemsUnselected
library call`XmListAddItemsUnselected`A List function that adds items
to a listXmListAddItemsUnselectedList functionsXmListAddItemsUnselected#include <Xm/List.h>void`XmListAddItemsUnselected`WidgetwidgetXmString *itemsintitem_countintposition
## DESCRIPTION


`XmListAddItemsUnselected`adds the specified items to the
list at the given position. The inserted items remain unselected,
even if they currently appear in the`XmNselectedItems`list.

* **`widget`** 

Specifies the ID of the List widget to add items to.
* **`items`** 

Specifies a pointer to the items to be added to the
list.
* **`item_count`** 

Specifies the number of elements in`items`.
This number must be nonnegative.
* **`position`** 

Specifies the position of the first new item in the list. A value
of 1 makes the first new item the first item in the list; a value
of 2 makes it the second item; and so on. A value of 0 (zero)
makes the
first new item follow the last item of the list.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.