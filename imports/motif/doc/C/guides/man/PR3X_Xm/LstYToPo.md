# XmListYToPos
library call`XmListYToPos`A List function that returns the position of the item at a specified y-coordinateXmListYToPosList functionsXmListYToPos#include <Xm/List.h>int`XmListYToPos`WidgetwidgetPositiony
## DESCRIPTION


`XmListYToPos`returns the position of the item
at the given y-coordinate within the list.

* **`widget`** 

Specifies the ID of the List widget
* **y** 

Specifies the y-coordinate in the list's coordinate system


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns the position of the item at the specified y
coordinate. A value of 1 indicates the first item in the
list; a value of 2 indicates the second item; and so on. A
value of 0 (zero) indicates that no item exists at the specified
y coordinate.
## RELATED


&cdeman.XmList; and &cdeman.XmListPosToBounds;.