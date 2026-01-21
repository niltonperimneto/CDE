# XmTrackingEvent
library call`XmTrackingEvent`A Toolkit function that provides a modal interactionXmTrackingEventToolkit functionsXmTrackingEvent#include <Xm/Xm.h>Widget`XmTrackingEvent`WidgetwidgetCursorcursorBooleanconfine_toXEvent *event_return
## DESCRIPTION


`XmTrackingEvent`provides a modal interface for selection of a
component.
It is intended to support context help.
The function
calls the`XmUpdateDisplay`function.`XmTrackingEvent`then grabs
the pointer and discards succeeding events until`BSelect`is released or a key is pressed and then released.
The function then returns the widget or gadget that contains the
pointer when`BSelect`is released or a key is
released, and ungrabs the pointer.

* **`widget`** 

Specifies the widget ID of a widget to use as the basis of the modal
interaction. That is, the widget within which the interaction must
occur, usually a top-level shell.
* **`cursor`** 

Specifies the cursor to be used for the pointer during the interaction.
This is a standard X cursor name.
* **`confine_to`** 

Specifies whether or not the cursor should be confined to`widget`.
* **`event_return`** 

Returns the ButtonRelease or KeyRelease event that causes the function
to return.

## RETURN


Returns the widget or gadget that contains the pointer when`BSelect`is released or a key is released. If no widget or gadget contains the
pointer, the function returns NULL.
## RELATED


&cdeman.XmTrackingLocate;.