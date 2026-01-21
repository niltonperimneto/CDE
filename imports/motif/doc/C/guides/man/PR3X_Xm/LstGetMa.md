# XmListGetMatchPos
library call`XmListGetMatchPos`A List function that returns all instances of an item in the listXmListGetMatchPosList functionsXmListGetMatchPos#include <Xm/List.h>Boolean`XmListGetMatchPos`WidgetwidgetXmStringitemint **position_listint *position_count
## DESCRIPTION


`XmListGetMatchPos`is a Boolean function that returns an array of
positions where a specified item is found in a List.

* **`widget`** 

Specifies the ID of the List widget.
* **`item`** 

Specifies the item to search for.
* **`position_list`** 

Returns an array of positions at which the item occurs in the List.
The position of the first item in the list is 1; the position of the
second item is 2; and so on.
When the return value is True,`XmListGetMatchPos`allocates
memory for this array.
The caller is responsible for freeing this memory.
The caller can recover the allocated memory by calling`XtFree`.
* **`position_count`** 

Returns the number of elements in the`position_list`.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns True if the specified item is present in the list, and
False if it is not.
## RELATED


&cdeman.XmList;.