# XmListDeletePos
library call`XmListDeletePos`A List function that deletes an item from a list at a specified positionXmListDeletePosList functionsXmListDeletePos#include <Xm/List.h>void`XmListDeletePos`Widgetwidgetintposition
## DESCRIPTION


`XmListDeletePos`deletes an item at a specified position.
A warning message appears if the position does not exist.

* **`widget`** 

Specifies the ID of the List from which an item is to be deleted.
* **`position`** 

Specifies the position of the item to be deleted.
A value of 1 indicates that the first item in the list is deleted; a
value of 2 indicates that the second item is deleted; and so on.
A value of 0 (zero)
indicates that the last item in the list is deleted.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.