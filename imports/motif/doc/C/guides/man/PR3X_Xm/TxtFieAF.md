# XmTextFieldGetBaseline
library call`XmTextFieldGetBaseline`A TextField function that accesses the y position of the baselineXmTextFieldGetBaselineTextField functionsXmTextFieldGetBaseline#include <Xm/TextF.h>int`XmTextFieldGetBaseline`Widgetwidget
## DESCRIPTION


`XmTextFieldGetBaseline`accesses theyposition of the
baseline in the TextField widget,
relative to theyposition of the top of the widget.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns an integer value that indicates theyposition
of the baseline in the TextField widget.
The calculation takes into account the
margin height, shadow thickness, highlight thickness, and font ascent of
the first font (set) in the fontlist used for drawing text.
In this calculation, theyposition of the top of the widget is 0 (zero).
## RELATED


&cdeman.XmTextField;.