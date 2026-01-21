# XmListPosToBounds
library call`XmListPosToBounds`A List function that returns the bounding
box of an item at a specified position in a listXmListPosToBoundsList functionsXmListPosToBounds#include <Xm/List.h>Boolean`XmListPosToBounds`WidgetwidgetintpositionPosition *xPosition *yDimension *widthDimension *height
## DESCRIPTION


`XmListPosToBounds`returns the coordinates
of an item within a list and the dimensions of its bounding
box. The function returns the associated x and y-coordinates
of the upper left corner of the bounding box relative to the
upper left corner of the List widget, as well as the width
and the height of the box. The caller can pass a NULL value
for thex,y,`width`, or`height`parameters
to indicate that the return value for that parameter is not
requested.

* **`widget`** 

Specifies the ID of the List widget.
* **`position`** 

Specifies the position of the specified item.
A value of 1 indicates the first item in
the list; a value of 2 indicates the second item; and so on. A
value of 0 (zero) specifies the last item in the list.
* **x** 

Specifies a pointer to the returned x-coordinate of the item.
* **y** 

Specifies the pointer to the returned y-coordinate of the item.
* **`width`** 

Specifies the pointer to the returned width of the item.
* **`height`** 

Specifies the pointer to the returned height of the item.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


If the item at the specified position is not visible,
returns False, and the returned values (if any) are undefined. Otherwise,
this function returns True.
## RELATED


&cdeman.XmList; and
&cdeman.XmListYToPos;.