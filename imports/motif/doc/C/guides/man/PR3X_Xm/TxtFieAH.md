# XmTextFieldGetInsertionPosition
library call`XmTextFieldGetInsertionPosition`A TextField function that accesses the position of the insertion cursorXmTextFieldGetInsertion\\%PositionTextField functionsXmTextFieldGetInsertion\\%Position#include <Xm/TextF.h>XmTextPosition`XmTextFieldGetInsertionPosition`Widgetwidget
## DESCRIPTION


`XmTextFieldGetInsertionPosition`accesses the insertion cursor
position of the TextField widget.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns anXmTextPositionvalue that indicates the state of the`XmNcursorPosition`resource. This is an integer number of
characters from the beginning of the text buffer. The first character
position is 0 (zero).
## RELATED


&cdeman.XmTextField;.