# XmTextFieldShowPosition
library call`XmTextFieldShowPosition`A TextField function that forces text at a given position to be displayedXmTextFieldShowPositionTextField functionsXmTextFieldShowPosition#include <Xm/TextF.h>void`XmTextFieldShowPosition`WidgetwidgetXmTextPositionposition
## DESCRIPTION


`XmTextFieldShowPosition`forces text at the specified position to be
displayed.
The cursor position is not updated nor is the cursor shown at this position.

* **`widget`** 

Specifies the TextField widget ID
* **`position`** 

Specifies the character position to be displayed. This is an integer
number of characters from the beginning of the text buffer. The first
character position is 0 (zero).
See &cdeman.XmTextPosition; for details on theXmTextPositiondata type.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField;
and
&cdeman.XmTextPosition;.