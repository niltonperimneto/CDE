# XmListReplaceItems
library call`XmListReplaceItems`A List function that replaces the specified elements in the listXmListReplaceItemsList functionsXmListReplaceItems#include <Xm/List.h>void`XmListReplaceItems`WidgetwidgetXmString *old_itemsintitem_countXmString *new_items
## DESCRIPTION


`XmListReplaceItems`replaces each specified item of the list
with a corresponding new item.
When the items are inserted into the list, they are compared with
the current`XmNselectedItems`list. If any of the new items
matches an item on the selected list, it appears selected.

* **`widget`** 

Specifies the ID of the List widget.
* **`old_items`** 

Specifies the items to be replaced.
* **`item_count`** 

Specifies the number of items in`old_items`and`new_items`.
This number must be nonnegative.
* **`new_items`** 

Specifies the replacement items.


Every occurrence of each element of`old_items`is replaced with
the corresponding element from`new_items`.
That is, the first element of`old_items`is replaced with the first
element of`new_items`. The second element of`old_items`is replaced
with the second element of`new_items`, and so on until`item_count`is reached.

For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.