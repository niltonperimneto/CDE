# XmRenderTableCvtToProp
library call`XmRenderTableCvtToProp`A render table function that converts a render table to a string representationXmRenderTableCvtToProprender table functionsXmRenderTableCvtToProp#include <Xm/Xm.h>unsigned int`XmRenderTableCvtToProp`WidgetwidgetXmRenderTabletablechar **prop_return
## DESCRIPTION


`XmRenderTableCvtToProp`converts a render table to a string of
characters representing the render table.
This routine is typically used by the source of a data transfer
operation to produce a representation for transferring a render table to
a destination.

* **`widget`** 

Specifies the widget that is the source of the render table
* **`table`** 

Specifies a render table to be converted
* **`prop_return`** 

Specifies a pointer to a string that is created and returned by this
function.
The function allocates space to hold the returned string.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XtFree`.

## RETURN


Returns the number of bytes in the string representation.
## RELATED


&cdeman.XmRenderTable; and
&cdeman.XmRenderTableCvtFromProp;.