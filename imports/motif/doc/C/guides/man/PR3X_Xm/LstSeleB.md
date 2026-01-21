# XmListSelectPos
library call`XmListSelectPos`A List function that selects an item at a specified position in the listXmListSelectPosList functionsXmListSelectPos#include <Xm/List.h>void`XmListSelectPos`WidgetwidgetintpositionBooleannotify
## DESCRIPTION


`XmListSelectPos`highlights a List item at the specified position
and adds it to the list of selected items.

* **`widget`** 

Specifies the ID of the List widget.
* **`position`** 

Specifies the position of the item to be selected.
A value of 1 indicates that the first item in the list is selected; a
value of 2 indicates that the second item is selected; and so on.
A value of 0 (zero) indicates that the last item in the list is selected.
* **`notify`** 

Specifies a Boolean value that when TRUE invokes the selection callback
for the current mode. From an application interface view, calling this
function with`notify`True is indistinguishable from a user-initiated
selection action.
When`notify`is FALSE, no callbacks are called.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList; and
&cdeman.XmListSelectItem;.