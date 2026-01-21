# XmTextSetInsertionPosition
library call`XmTextSetInsertionPosition`A Text function that sets the position of the insert cursorXmTextSetInsertionPositionText functionsXmTextSetInsertionPosition#include <Xm/Text.h>void`XmTextSetInsertionPosition`WidgetwidgetXmTextPositionposition
## DESCRIPTION


`XmTextSetInsertionPosition`sets the insertion cursor position of the
Text widget.
This routine also calls the widget's`XmNmotionVerifyCallback`callbacks if the insertion cursor position changes.

* **`widget`** 

Specifies the Text widget ID
* **`position`** 

Specifies the position of the insertion cursor. This is an integer number
of characters from the beginning of the text buffer. The first
character position is 0 (zero).


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.