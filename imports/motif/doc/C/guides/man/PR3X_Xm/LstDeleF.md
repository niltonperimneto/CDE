# XmListDeletePositions
library call`XmListDeletePositions`A List function that deletes items
from a list based on an array of positionsXmListDeletePositionsList functionsXmListDeletePositions#include <Xm/List.h>void`XmListDeletePositions`Widgetwidgetint *position_listintposition_count
## DESCRIPTION


`XmListDeletePositions`deletes noncontiguous
items from a list. The function deletes all items whose
corresponding positions appear in the`position_list`array.
A warning message is displayed if a specified position is invalid;
that is, the value is 0, a negative integer, or a number greater
than the number of items in the list.

* **`widget`** 

Specifies the ID of the List widget
* **`position_list`** 

Specifies an array of the item positions to be deleted. The
position of the first item in the list is 1; the position of
the second item is 2; and so on.
* **`position_count`** 

Specifies the number of elements in the`position_list`.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.