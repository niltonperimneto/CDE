# XmTabListCopy
library call`XmTabListCopy`A convenience function that creates a new tab list from an existing listXmTabListCopy#include <Xm/Xm.h>XmTabList`XmTabListCopy`XmTabListtablistintoffsetCardinalcount
## DESCRIPTION


`XmTabListCopy`creates a new tab list consisting of a copy of
a portion of the contents of the`tablist`argument. This
function starts copying at the specified offset value of the tab list
and copies`count`values.

* **`tablist`** 

Specifies a tab list to be copied.
* **`offset`** 

Specifies where to start copying. A value of 0 (zero) indicates begin
at the beginning, a value of 1 indicates to skip the first tab, and so
on. A negative indicates to begin counting backwards from the end.
A value of -1 indicates to start copying from the last tab.
* **`count`** 

Specifies the number of tabs to copy. A value of 0 (zero) indicates
to copy all elements from the starting point to the end (beginning if`offset`is negative) of the tab list.

## RETURN


If`tablist`is NULL, this function returns NULL. Otherwise,
this function returns a newly allocatedXmTabList.
If the function does allocate anXmTabList, then the
application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmTabListFree`.
## RELATED


&cdeman.XmTabList; and
&cdeman.XmTabListFree;.