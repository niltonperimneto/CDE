# XmTextGetInsertionPosition
library call`XmTextGetInsertionPosition`A Text function that accesses the position of the insert cursorXmTextGetInsertionPositionText functionsXmTextGetInsertionPosition#include <Xm/Text.h>XmTextPosition`XmTextGetInsertionPosition`Widgetwidget
## DESCRIPTION


`XmTextGetInsertionPosition`accesses the insertion cursor position of the
Text widget.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns anXmTextPositionvalue that indicates the state of the`XmNcursorPosition`resource. This is an integer number of
characters from the beginning of the text buffer. The first character
position is 0 (zero).
## RELATED


&cdeman.XmText;.