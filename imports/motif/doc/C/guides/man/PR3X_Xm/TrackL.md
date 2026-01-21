# XmTrackingLocate
library call`XmTrackingLocate`A Toolkit function that provides a modal interactionXmTrackingLocateToolkit functionsXmTrackingLocate#include <Xm/Xm.h>Widget`XmTrackingLocate`WidgetwidgetCursorcursorBooleanconfine_to
## DESCRIPTION


`XmTrackingLocate`provides a modal interface for selection of a
component.
It is intended to support context help.
This function is implemented as`XmTrackingEvent`.

`NOTE:`This function is obsolete and exists for compatibility with
previous releases. It has been replaced by`XmTrackingEvent`.

* **`widget`** 

Specifies the widget ID of a widget to use as the basis of the modal
interaction. That is, the widget within which the interaction must
occur, usually a top-level shell.
* **`cursor`** 

Specifies the cursor to be used for the pointer during the interaction.
This is a standard X cursor name.
* **`confine_to`** 

Specifies whether or not the cursor should be confined to`widget`

## RETURN


Returns the widget or gadget that contains the pointer when`BSelect`is released or a key is released. If no widget or gadget contains the
pointer, the function returns NULL.
## RELATED


&cdeman.XmTrackingEvent;.