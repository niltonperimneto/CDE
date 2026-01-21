# XmListReplacePositions
library call`XmListReplacePositions`A List function that replaces items
in a list based on positionXmListReplacePositionsList functionsXmListReplacePositions#include <Xm/List.h>void`XmListReplacePositions`Widgetwidgetint *position_listXmString *item_listintitem_count;
## DESCRIPTION


`XmListReplacePositions`replaces noncontiguous items
in a list. The item at each position specified in`position_list`is replaced with the corresponding entry in`item_list`.
When the items are inserted into the list, they are compared with the
current`XmNselectedItems`list. Any of the new items that match
items on the selected list appear selected. A warning message
is displayed if a specified position is invalid; that is, the value is 0 (zero),
a negative integer, or a number greater than the number of items
in the list.

* **`widget`** 

Specifies the ID of the List widget.
* **`position_list`** 

Specifies an array of the positions of items to be replaced. The
position of the first item in the list is 1; the position of the
second item is 2; and so on.
* **`item_list`** 

Specifies an array of the replacement items.
* **`item_count`** 

Specifies the number of elements in`position_list`and`item_list`.
This number must be nonnegative.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.