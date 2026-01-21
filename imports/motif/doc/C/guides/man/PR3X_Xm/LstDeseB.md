# XmListDeselectItem
library call`XmListDeselectItem`A List function that deselects the specified item from the selected listXmListDeselectItemList functionsXmListDeselectItem#include <Xm/List.h>void`XmListDeselectItem`WidgetwidgetXmStringitem
## DESCRIPTION


`XmListDeselectItem`unhighlights and removes from the selected list
the first item in the list that matches`item`.

* **`widget`** 

Specifies the ID of the List from whose list an item is deselected.
* **`item`** 

Specifies the item to be deselected from the list.
If`item`appears more than once in the List, only the
first occurrence is matched.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.