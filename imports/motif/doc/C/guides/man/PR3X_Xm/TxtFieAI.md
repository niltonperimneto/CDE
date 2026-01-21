# XmTextFieldGetLastPosition
library call`XmTextFieldGetLastPosition`A TextField function that accesses the position of the last text characterXmTextFieldGetLastPositionTextField functionsXmTextFieldGetLastPosition#include <Xm/TextF.h>XmTextPosition`XmTextFieldGetLastPosition`Widgetwidget
## DESCRIPTION


`XmTextFieldGetLastPosition`accesses the position of the
last character in the text buffer of the TextField widget.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns anXmTextPositionvalue that indicates the position of the last
character in the text buffer. This is an integer number of
characters from the beginning of the buffer. The first character
position is 0 (zero).
The last character position is equal to the number of characters
in the text buffer.
## RELATED


&cdeman.XmTextField;.