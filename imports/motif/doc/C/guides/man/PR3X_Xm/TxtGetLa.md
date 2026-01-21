# XmTextGetLastPosition
library call`XmTextGetLastPosition`A Text function that accesses the last position in the textXmTextGetLastPositionText functionsXmTextGetLastPosition#include <Xm/Text.h>XmTextPosition`XmTextGetLastPosition`Widgetwidget
## DESCRIPTION


`XmTextGetLastPosition`accesses the last position
in the text buffer of the Text widget. This is an integer
number of characters from the beginning of the buffer, and
represents the position that text added to the end
of the buffer is placed after. The first character position is 0 (zero).
The last character position is equal to the number of characters
in the text buffer.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns anXmTextPositionvalue that indicates the last position
in the text buffer.
## RELATED


&cdeman.XmText;.