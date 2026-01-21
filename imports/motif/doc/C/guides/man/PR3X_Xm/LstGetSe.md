# XmListGetSelectedPos
library call`XmListGetSelectedPos`A List function that returns the position of every selected item in the listXmListGetSelectedPosList functionsXmListGetSelectedPos#include <Xm/List.h>Boolean`XmListGetSelectedPos`Widgetwidgetint **position_listint *position_count
## DESCRIPTION


This routine is obsolete. It is
replaced by calling`XtGetValues`for the List resources`XmNselectedPositions`and`XmNselectedPositionCount`.`XmListGetSelectedPos`is a Boolean function that returns an
array of the positions of the selected items in a List.

* **`widget`** 

Specifies the ID of the List widget.
* **`position_list`** 

Returns an array of the positions of the selected items in the List.
The position of the first item in the list is 1; the position of the
second item is 2; and so on.
When the return value is True,`XmListGetSelectedPos`allocates
memory for this array.
The caller is responsible for freeing this memory.
The caller can recover the allocated memory by calling`XtFree`.
* **`position_count`** 

Returns the number of elements in the`position_list`.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns True if the list has any selected items, and False if it does
not.
## RELATED


&cdeman.XmList;.