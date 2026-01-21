# XmGetXmDisplay
library call`XmGetXmDisplay`A Display function that returns the
XmDisplay object ID for a specified displayXmGetXmDisplayDisplay functionsXmGetXmDisplay#include <Xm/Display.h>Widget`XmGetXmDisplay`Display *display
## DESCRIPTION


`XmGetXmDisplay`returns the`XmDisplay`object ID associated
with a display. The application can access
Display resources with`XtGetValues`.

* **`display`** 

Specifies the display for which the`XmDisplay`object ID is to be returned


For a complete definition of Display and its associated resources,
see &cdeman.XmDisplay;.
## RETURN


Returns the`XmDisplay`object ID for the specified display.
## RELATED


&cdeman.XmDisplay;.