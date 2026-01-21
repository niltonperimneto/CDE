# XmTabListReplacePositions
library call`XmTabListReplacePositions`A convenience function that creates a new tab list with replacement tabsXmTabListReplacePositions#include <Xm/Xm.h>XmTabList`XmTabListReplacePositions`XmTabListoldlistCardinal*position_listXmTab*tabsCardinaltab_count
## DESCRIPTION


`XmTabListReplacePositions`creates a new tab list that contains
the contents of`oldlist`, but with the tabs at the positions in`position_list`replaced with copies of the corresponding tabs in`tabs`.
A warning message is displayed if a specified position is invalid;
for example, if the value is a number greater than the number of tabs
in the tab list.

This function deallocates the original tab list after extracting the
required information. It is the caller's responsibility to free the
tabs in`tabs`by using the`XmTabFree`function.

* **`oldlist`** 

Specifies the tab list.
The function deallocates the tab list after extracting the required
information.
* **`position_list`** 

Specifies an array of positions of the tabs to be replaced. The
position of the first tab is 0 (zero), the position of the second tab
is 1, and so on.
* **`tabs`** 

Specifies an array of the replacement tabs.
* **`tab_count`** 

Specifies the number of elements in`position_list`and`tabs`.

## RETURN


If`tabs`,`oldlist`, or`position_list`is NULL, or`tab_count`is 0 (zero), returns`oldlist`. Otherwise, this function returns the new tab list.
The function allocates space to hold the returned tab list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmTabListFree`.
## RELATED


&cdeman.XmTabList;.