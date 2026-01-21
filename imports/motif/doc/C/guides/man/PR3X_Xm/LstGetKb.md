# XmListGetKbdItemPos
library call`XmListGetKbdItemPos`A List function that returns the
position of the item at the location cursorXmListGetKbdItemPosList functionsXmListGetKbdItemPos#include <Xm/List.h>int`XmListGetKbdItemPos`Widgetwidget
## DESCRIPTION


`XmListGetKbdItemPos`returns the position of the list
item at the location cursor.

* **`widget`** 

Specifies the ID of the List widget


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns the position of the current keyboard item. A value of 1
indicates that the location cursor is at the first item of the
list; a value of 2 indicates that it is at the second item; and
so on. A value of 0 (zero) indicates the List widget is empty.
## RELATED


&cdeman.XmList;.