# XmListAddItem
library call`XmListAddItem`A List function that adds an item to the listXmListAddItemList functionsXmListAddItem#include <Xm/List.h>void`XmListAddItem`WidgetwidgetXmStringitemintposition
## DESCRIPTION


`XmListAddItem`adds an item to the list at the given position.
When the item is inserted into the list, it is compared with the current`XmNselectedItems`list.
If the new item matches an item on the selected list, it appears
selected.

* **`widget`** 

Specifies the ID of the List to which an item is added.
* **`item`** 

Specifies the item to be added to the list.
* **`position`** 

Specifies the position of the new item in the list.
A value of 1 makes the new item the first item in the list; a value of 2
makes it the second item; and so on.
A value of 0 (zero) makes the new item the last item in the list.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.