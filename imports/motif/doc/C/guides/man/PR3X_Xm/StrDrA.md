# XmStringDraw
library call`XmStringDraw`A compound string function that draws a compound string in an X windowXmStringDrawcompound string functionsXmStringDraw#include <Xm/Xm.h>void`XmStringDraw`Display* dWindowwXmRenderTablerendertableXmStringstringGCgcPositionxPositionyDimensionwidthunsigned charalignmentunsigned charlayout_directionXRectangle* clip
## DESCRIPTION


`XmStringDraw`draws a compound string in an X Window.
If a compound string segment uses a
rendition
that
contains
a font set, the graphic context passed to this
routine will have the GC font member left in an undefined
state. The underlying`XmbStringDraw`function called
by this routine modifies the font ID field of the GC passed
into it and does not attempt to restore the font ID to the
incoming value. If the compound string segment is not drawn
using a font set, the graphic context must contain a valid font
member. Graphic contexts created by`XtGetGC`are not
valid for this routine; instead, use`XtAllocateGC`to create a graphic context.

* **`d`** 

Specifies the display.
* **`w`** 

Specifies the window.
* **`rendertable`** 

Specifies the render table.
* **`string`** 

Specifies the string.
* **`gc`** 

Specifies the graphics context to use.
* **x** 

Specifies a coordinate of the rectangle that will contain the displayed
compound string.
* **y** 

Specifies a coordinate of the rectangle that will contain the displayed
compound string.
* **`width`** 

Specifies the width of the rectangle that will contain the
displayed compound string.
* **`alignment`** 

Specifies how the string will be aligned within the specified rectangle.
It is either`XmALIGNMENT_BEGINNING`,`XmALIGNMENT_CENTER`, or`XmALIGNMENT_END`.
* **`layout_direction`** 

Controls the direction in which the segments of the compound string will
be laid out. It also determines the meaning of the`alignment`parameter.
* **`clip`** 

Allows the application to restrict the area into which the compound string
will be drawn.
If the value is NULL, clipping will be determined by the GC.

## RELATED


&cdeman.XmStringCreate;.