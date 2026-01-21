# XmChangeColor
library call`XmChangeColor`Recalculates all associated colors of
a widgetXmChangeColorColor functionsXmChangeColor#include <Xm/Xm.h>void`XmChangeColor`WidgetwidgetPixelbackground
## DESCRIPTION


`XmChangeColor`handles all color modifications for the
specified widget when a new background pixel value is specified.
This function recalculates the foreground, select, and shadow
colors based on the new background color and sets the corresponding
resources for the widget. If a color calculation procedure has
been set by a call to`XmSetColorCalculation`,`XmChangeColor`uses that procedure to calculate the new colors. Otherwise, the
routine uses a default procedure.

* **`widget`** 

Specifies the widget ID whose colors will be updated
* **`background`** 

Specifies the background color pixel value

## RELATED


&cdeman.XmGetColorCalculation;,
&cdeman.XmGetColors;, and
&cdeman.XmSetColorCalculation;.