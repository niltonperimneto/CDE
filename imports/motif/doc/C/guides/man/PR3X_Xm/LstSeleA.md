# XmListSelectItem
library call`XmListSelectItem`A List function that selects an item in the listXmListSelectItemList functionsXmListSelectItem#include <Xm/List.h>void`XmListSelectItem`WidgetwidgetXmStringitemBooleannotify
## DESCRIPTION


`XmListSelectItem`highlights and adds to the selected list the
first item in the list that matches`item`.

* **`widget`** 

Specifies the ID of the List widget from whose list an item is selected.
* **`item`** 

Specifies the item to be selected in the List widget.
If`item`appears more than once in the List, only the
first occurrence is matched.
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
&cdeman.XmListSelectPos;.