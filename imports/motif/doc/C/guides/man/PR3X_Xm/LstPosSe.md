# XmListPosSelected
library call`XmListPosSelected`A List function that determines if the
list item at a specified position is selectedXmListPosSelectedList functionsXmListPosSelected#include <Xm/List.h>Boolean`XmListPosSelected`Widgetwidgetintposition
## DESCRIPTION


`XmPosSelected`determines if the list item at the specified
position is selected or not.

* **`widget`** 

Specifies the ID of the List widget
* **`position`** 

Specifies the position of the list item. A value of 1 indicates
the first item in the list; a value of 2 indicates the second item;
and so on. A value of 0 (zero) specifies the last item in the list.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns True if the list item is selected; otherwise, returns False
if the item is not selected or the specified position is invalid.
## RELATED


&cdeman.XmList;.