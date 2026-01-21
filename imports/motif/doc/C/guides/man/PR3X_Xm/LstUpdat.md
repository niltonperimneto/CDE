# XmListUpdateSelectedList
library call`XmListUpdateSelectedList`A List function that updates
the XmNselectedItems resourceXmListUpdateSelectedListList functionsXmListUpdateSelectedList#include <Xm/List.h>void`XmListUpdateSelectedList`Widgetwidget
## DESCRIPTION


`XmListUpdateSelectedList`frees the contents of the
current`XmNselectedItems`list. The routine traverses
the`XmNitems`list and adds each currently selected item to
the`XmNselectedItems`list. For each selected item, there is
a corresponding entry in the updated`XmNselectedItems`list.

* **`widget`** 

Specifies the ID of the List widget to update


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.