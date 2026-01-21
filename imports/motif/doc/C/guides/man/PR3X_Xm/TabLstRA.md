# XmTabListRemoveTabs
library call`XmTabListRemoveTabs`A convenience function that removes noncontiguous tabsXmTabListRemoveTabs#include <Xm/Xm.h>XmTabList`XmTabListRemoveTabs`XmTabListoldlistCardinal*position_listCardinalposition_count
## DESCRIPTION


`XmTabListRemoveTabs`removes noncontiguous tabs from a tab list.
The function creates a new tab list by copying the contents of`oldlist`and removing all tabs whose corresponding positions
appear in the`position_list`array.
A warning message
is displayed if a specified position is invalid; for example, if the
value is a number greater than the number of tabs in the tab list.

* **`tablist`** 

Specifies the tab list.
The function deallocates`oldlist`and the tabs it contains
after extracting the required information.
* **`position_list`** 

Specifies an array of the tab positions to be removed. The position
of the first tab in the list is 0 (zero), the position of the second
tab is 1, and so on.
* **`position_count`** 

Specifies the number of elements in the`position_list`.

## RETURN


If`oldlist`or`position_list`is NULL, or`position_count`is 0 (zero), returns`oldlist`. Otherwise,
this function returns the new tab list.
The function allocates space to hold the returned tab list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmTabListFree`.
## RELATED


&cdeman.XmTabList; and
&cdeman.XmTabListFree;.