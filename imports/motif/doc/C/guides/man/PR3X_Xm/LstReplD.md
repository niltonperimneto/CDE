# XmListReplaceItemsUnselected
library call`XmListReplaceItemsUnselected`A List function that replaces items
in a listXmListReplaceItemsUn\\%selectedList functionsXmListReplaceItemsUn\\%selected#include <Xm/List.h>void`XmListReplaceItemsUnselected`WidgetwidgetXmString *old_itemsintitem_countXmString *new_items
## DESCRIPTION


`XmListReplaceItemsUnselected`replaces each specified item in the
list with a corresponding new item. The replacement items
remain unselected, even if they currently appear in the`XmNselectedItems`list.

* **`widget`** 

Specifies the ID of the List widget to replace items in.
* **`old_items`** 

Specifies a pointer to the list items to be replaced.
* **`item_count`** 

Specifies the number of elements in`old_items`and`new_items`.
This number must be nonnegative.
* **`new_items`** 

Specifies a pointer to the replacement items. Every
occurrence of each element of`old_items`is replaced
with the corresponding element from`new_items`. That
is, the first element of`old_items`is replaced with
the first element of`new_items`. The second element
of`old_items`is replaced with the second element of`new_items`, and so on until`item_count`is
reached. If an element in`old_items`does not exist
in the list, the corresponding entry in`new_items`is skipped.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.