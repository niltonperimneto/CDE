# XmGetFocusWidget
library call`XmGetFocusWidget`Returns the ID of the widget that has
keyboard focusXmGetFocusWidgettraversal functionsXmGetFocusWidget#include <Xm/Xm.h>Widget`XmGetFocusWidget`Widgetwidget
## DESCRIPTION


`XmGetFocusWidget`examines the hierarchy that contains
the specified widget and returns the ID of the widget that
has keyboard focus. The function extracts the widget ID
from the associated Shell widget; therefore, the specified
widget can be located anywhere in the hierarchy.

* **`widget`** 

Specifies a widget ID within a given hierarchy

## RETURN


Returns the ID of the widget with keyboard focus. If no child
of the Shell has focus, the function returns NULL.
## RELATED


&cdeman.XmProcessTraversal;.