# XmTabListGetTab
library call`XmTabListGetTab`A convenience function that returns a copy of a tabXmTabListGetTab#include <Xm/Xm.h>XmTab`XmTabListGetTab`XmTabListtablistCardinalposition
## DESCRIPTION


`XmTabListGetTab`returns a copy of the tab that is located at the
specified position in the tab list.

* **`tablist`** 

Specifies the tab list.
* **`position`** 

Specifies the position of the tab to be returned. A value of 0 (zero)
returns the first tab in the tab list, a value of 1 returns the second
tab, and so on.

## RETURN


Returns a copy of the tab that is located at the specified position in
the tab list. If`position`is greater than or equal to the
number of tabs in the tab list, this function returns NULL.
The application is responsible for managing the space allocted by
the returned tab copy. The application can recover this allocated
space by calling`XmTabFree`.
## RELATED


&cdeman.XmTabFree; and
&cdeman.XmTabList;.