# XmListReplaceItemsPosUnselected
library call`XmListReplaceItemsPosUnselected`A List function that replaces items
in a list without selecting the replacement itemsXmListReplaceItemsPosUn\\%selectedList functionsXmListReplaceItemsPosUn\\%selected#include <Xm/List.h>void`XmListReplaceItemsPosUnselected`WidgetwidgetXmString *new_itemsintitem_countintposition
## DESCRIPTION


`XmListReplaceItemsPosUnselected`replaces the specified number of
items in the list with new items, starting at the given position. The
replacement items remain unselected, even if they currently appear in
the`XmNselectedItems`list.

* **`widget`** 

Specifies the ID of the List widget to replace items in.
* **`new_items`** 

Specifies a pointer to the replacement items.
* **`item_count`** 

Specifies the number of elements in`new_items`and the
number of items in the list to replace.
This number must be nonnegative.
* **`position`** 

Specifies the position of the first item in the list to
be replaced. A value of 1 indicates that the first item
replaced is the first item in the list; a value of 2
indicates that it is the second item; and so on.

Beginning with the item specified in`position`,`item_count`items in the list are replaced with
the corresponding elements from`new_items`. That
is, the item at`position`is replaced with the
first element of`new_items`; the item after`position`is replaced with the second element
of`new_items`; and so on, until`item_count`is reached.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.