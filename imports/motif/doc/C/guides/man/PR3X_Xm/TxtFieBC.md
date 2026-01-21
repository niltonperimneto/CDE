# XmTextFieldSetInsertionPosition
library call`XmTextFieldSetInsertionPosition`A TextField function that sets the position of the insertion cursorXmTextFieldSetInsertion\\%PositionTextField functionsXmTextFieldSetInsertion\\%Position#include <Xm/TextF.h>void`XmTextFieldSetInsertionPosition`WidgetwidgetXmTextPositionposition
## DESCRIPTION


`XmTextFieldSetInsertionPosition`sets the insertion cursor position
of the TextField widget.
This routine also calls the widget's`XmNmotionVerifyCallback`callbacks if the insertion cursor position changes.

* **`widget`** 

Specifies the TextField widget ID
* **`position`** 

Specifies the position of the insert cursor. This is an integer number
of characters from the beginning of the text buffer. The first
character position is 0 (zero).


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField;.