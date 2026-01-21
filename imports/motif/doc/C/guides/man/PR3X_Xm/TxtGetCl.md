# XmTextGetCenterline
library call`XmTextGetCenterline`Return the height (length) of a character string
when the writing direction is vertical#include <Xm/Text.h>int`XmTextGetCenterline`Widgetwidget
## DESCRIPTION
XmTextGetCenterlineaccesses the x position of the centerline in theTextwidget, relative to the x position of the top of the widget.
`widget`Specifies theTextwidget ID.RETURN VALUEIn the case of horizontal writing, this function accesses 0.In the case of vertical writing, this function accesses the
x position of the first centerline in the Text widget,
relative to the x position of the left of the widget.
The calculation takes into account the margin width, shadow
thickness, highlight thickness, and a half of font width of
the first font(set) in the fontlist used for drawing text.SEE ALSO&cdeman.XmText;, &cdeman.XmTextGetBaseline;