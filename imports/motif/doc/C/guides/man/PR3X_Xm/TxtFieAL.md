# XmTextFieldGetSelectionPosition
library call`XmTextFieldGetSelectionPosition`A TextField function that accesses the position of the primary selectionXmTextFieldGetSelection\\%PositionTextField functionsXmTextFieldGetSelection\\%Position#include <Xm/TextF.h>Boolean`XmTextFieldGetSelectionPosition`WidgetwidgetXmTextPosition*leftXmTextPosition*right
## DESCRIPTION


`XmTextFieldGetSelectionPosition`accesses the left and right position of
the primary selection in the text buffer of the TextField widget.

* **`widget`** 

Specifies the TextField widget ID
* **`left`** 

Specifies the pointer in which the position of the left boundary of the
primary selection is returned. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).
* **`right`** 

Specifies the pointer in which the position of the right boundary of the
primary selection is returned. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


This function returns True if the widget owns the primary selection;
otherwise, it returns False.
## RELATED


&cdeman.XmTextField;.