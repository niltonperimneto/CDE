# XmScrollVisible
library call`XmScrollVisible`A ScrolledWindow function that makes an invisible
 descendant of a ScrolledWindow work area visibleXmScrollVisibleScrolled Window functionsXmScrollVisible#include <Xm/ScrolledW.h>void`XmScrollVisible`Widgetscrollw_widgetWidgetwidgetDimensionleft_right_marginDimensiontop_bottom_margin
## DESCRIPTION


`XmScrollVisible`makes an obscured or partially obscured
widget or gadget descendant of a ScrolledWindow work area visible. The
function repositions the work area and sets the specified margins
between the widget and the nearest viewport boundary. The widget's
location relative to the viewport determines whether one or both
of the margins must be adjusted. This function requires that
the`XmNscrollingPolicy`of the ScrolledWindow widget be set
to`XmAUTOMATIC`.

* **`scrollw_widget`** 

Specifies the ID of the ScrolledWindow widget whose work area
window contains an obscured descendant.
* **`widget`** 

Specifies the ID of the widget to be made visible.
* **`left_right_margin`** 

Specifies the margin to establish between the left
or right edge of the widget and the associated edge of the
viewport.
This margin is established only if the widget must be moved horizontally
to make it visible.
* **`top_bottom_margin`** 

Specifies the margin to establish between the top
or bottom edge of the widget and the associated edge of the
viewport.
This margin is established only if the widget must be moved vertically
to make it visible.


For a complete definition of ScrolledWindow and its associated
resources, see &cdeman.XmScrolledWindow;
## RELATED


&cdeman.XmScrolledWindow;.