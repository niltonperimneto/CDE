# XmTextGetBaseline
library call`XmTextGetBaseline`A Text function that accesses the y position of the baselineXmTextGetBaselineText functionsXmTextGetBaseline#include <Xm/Text.h>int`XmTextGetBaseline`Widgetwidget
## DESCRIPTION


`XmTextGetBaseline`accesses theyposition of the
baseline in the Text widget,
relative to theyposition of the top of the widget.

In vertical mode (when theXmNlayoutDirectionresource isXmTOP_TO_BOTTOM) this function returns 0 and
the program should use`XmTextGetCenterline`



* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns an integer value that indicates theyposition
of the baseline in the Text widget.
The calculation takes into account the
margin height, shadow thickness, highlight thickness, and font ascent of
the first font (set) in the fontlist used for drawing text.
In this calculation, theyposition of the top of the widget is 0 (zero).
## RELATED


&cdeman.XmText;, &cdeman.XmTextGetCenterline;.