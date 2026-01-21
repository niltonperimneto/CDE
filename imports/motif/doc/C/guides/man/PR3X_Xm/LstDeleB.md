# XmListDeleteItem
library call`XmListDeleteItem`A List function that deletes an item from the listXmListDeleteItemList functionsXmListDeleteItem#include <Xm/List.h>void`XmListDeleteItem`WidgetwidgetXmStringitem
## DESCRIPTION


`XmListDeleteItem`deletes the first item in the list that matches`item`.
A warning message appears if the item does not exist.

* **`widget`** 

Specifies the ID of the List from whose list an item is deleted.
* **`item`** 

Specifies the text of the item to be deleted from the list.
If`item`appears more than once in the List, only the
first occurrence is matched.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.