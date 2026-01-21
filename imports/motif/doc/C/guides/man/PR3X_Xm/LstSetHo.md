# XmListSetHorizPos
library call`XmListSetHorizPos`A List function that scrolls to the specified position in the listXmListSetHorizPosList functionsXmListSetHorizPos#include <Xm/List.h>void`XmListSetHorizPos`Widgetwidgetintposition
## DESCRIPTION


`XmListSetHorizPos`sets the`XmNvalue`resource of the
horizontal ScrollBar to the
specified position and updates the visible portion of the list with the
new value if the List widget's`XmNlistSizePolicy`is set to`XmCONSTANT`or`XmRESIZE_IF_POSSIBLE`and the horizontal ScrollBar is currently visible.
This is equivalent to moving the horizontal ScrollBar to the specified
position.

* **`widget`** 

Specifies the ID of the List widget
* **`position`** 

Specifies the horizontal position


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.