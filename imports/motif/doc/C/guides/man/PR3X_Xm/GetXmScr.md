# XmGetXmScreen
library call`XmGetXmScreen`A Screen function that returns the XmScreen object
ID for a specified screenXmGetXmScreen#include <Xm/Screen.h>Widget`XmGetXmScreen`Screen *screen
## DESCRIPTION


`XmGetXmScreen`returns the`XmScreen`object ID associated
with a screen. The application can access
and manipulate Screen resources with`XtGetValues`and`XtSetValues`.

* **`screen`** 

Specifies the screen for which the`XmScreen`ID is to be returned


For a complete definition of Screen and its associated resources,
see &cdeman.XmScreen;.
## RETURN


Returns the`XmScreen`object ID.
## RELATED


&cdeman.XmScreen;.