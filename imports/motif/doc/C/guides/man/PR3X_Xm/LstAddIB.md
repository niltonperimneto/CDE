# XmListAddItemUnselected
library call`XmListAddItemUnselected`A List function that adds an item to the listXmListAddItemUnselectedList functionsXmListAddItemUnselected#include <Xm/List.h>void`XmListAddItemUnselected`WidgetwidgetXmStringitemintposition
## DESCRIPTION


`XmListAddItemUnselected`adds an item to the list at the given
position.
The item does not appear selected, even if it matches an item in the
current`XmNselectedItems`list.

* **`widget`** 

Specifies the ID of the List from whose list an item is added.
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