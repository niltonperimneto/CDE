# XmRenderTableCvtFromProp
library call`XmRenderTableCvtFromProp`A render table function that converts from a string representation to a render tableXmRenderTableCvtFromProprender table functionsXmRenderTableCvtFromProp#include <Xm/Xm.h>XmRenderTable`XmRenderTableCvtFromProp`Widgetwidgetchar *propertyunsigned intlength
## DESCRIPTION


`XmRenderTableCvtFromProp`converts a string of characters representing a
render table to a render table.
This routine is typically used by the destination of a data transfer
operation to produce a render table from a transferred representation.

* **`widget`** 

Specifies the widget that is the destination for the render table
* **`property`** 

Specifies a string of characters representing a render table
* **`length`** 

Specifies the number of bytes in`property`

## RETURN


Returns a render table.
The function allocates space to hold the returned render table.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XmRenderTableFree`.
## RELATED


&cdeman.XmRenderTable;,
&cdeman.XmRenderTableCvtToProp;, and
&cdeman.XmRenderTableFree;.