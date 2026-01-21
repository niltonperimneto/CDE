# XmGetDestination
library call`XmGetDestination`A function that returns the widget ID of the widget to be used as the current destination for quick paste and certain clipboard operationsXmGetDestination#include <Xm/Xm.h>Widget`XmGetDestination`Display*display
## DESCRIPTION


`XmGetDestination`returns the widget that is the current destination
on the specified display.
The destination is generally the last editable widget on which a
select, edit, insert, or paste operation was performed and is the
destination for quick paste and certain clipboard functions.
The destination is NULL until a keyboard or mouse operation has been
done on an editable widget. Refer to the &MotifProgGd; for a description of
keyboard focus.

* **`display`** 

Specifies the display whose destination widget is to be queried

## RETURN


Returns the widget ID for the current destination or NULL if there is
no current destination.