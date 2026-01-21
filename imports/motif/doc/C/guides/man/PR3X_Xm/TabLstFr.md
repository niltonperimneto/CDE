# XmTabListFree
library call`XmTabListFree`A convenience function that frees the memory of a new tab listXmTabListFree#include <Xm/Xm.h>void`XmTabListFree`XmTabListtablist
## DESCRIPTION


`XmTabListFree`recovers memory used by a tab list. In addition,
this function frees all contained tabs. If the`tablist`is NULL,
the function returns immediately.

* **`tablist`** 

Specifies the tab list to be freed.

## RELATED


&cdeman.XmTabList;.