# XmTabListInsertTabs
library call`XmTabListInsertTabs`A convenience function that inserts tabs into a tab listXmTabListInsertTabs#include <Xm/Xm.h>XmTabList`XmTabListInsertTabs`XmTabListoldlistXmTab*tabsCardinaltab_countintposition
## DESCRIPTION


`XmTabListInsertTabs`creates a new tab list that includes the
tabs in`oldlist`. This function copies specified tabs to the
tab list at the given position. The first`tab_count`tabs of the`tabs`array are added to the tab list. If`oldlist`is NULL,`XmTabListInsertTabs`creates a new tab list containing only the
tabs specified.

* **`oldlist`** 

Specifies the tab list to add the tabs to.
The function deallocates`oldlist`after extracting the required information.
* **`tabs`** 

Specifies a pointer to the tabs to be added to the tab list.
It is the caller's responsibility to free the
tabs in`tabs`by using`XmTabFree`.
* **`tab_count`** 

Specifies the number of tabs in`tabs`.
* **`position`** 

Specifies the position of the first new tab in the tab list. A value
of 0 (zero) makes the first new tab the first tab in the tab list, a
value of 1 makes it the second tab, and so on. If`position`is
greater than the number of tabs in`oldlist`, then the tabs will
be inserted at the end. If`position`is negative, the count will
be backwards from the end. A value of -1 makes the first new tab
the last tab, and so on.

## RETURN


If`tabs`is NULL or`tab_count`is 0 (zero), this function returns`oldlist`. Otherwise, it returns a new tab list.
The function allocates space to hold the returned tab list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmTabListFree`.
## RELATED


&cdeman.XmTabList; and
&cdeman.XmTabListFree;.