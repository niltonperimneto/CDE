# XmTextGetSelectionPosition
library call`XmTextGetSelectionPosition`A Text function that accesses the position of the primary selectionXmTextGetSelectionPositionText functionsXmTextGetSelectionPosition#include <Xm/Text.h>Boolean`XmTextGetSelectionPosition`WidgetwidgetXmTextPosition*leftXmTextPosition*right
## DESCRIPTION


`XmTextGetSelectionPosition`accesses the left and right position of
the primary selection in the text buffer of the Text widget.

* **`widget`** 

Specifies the Text widget ID
* **`left`** 

Specifies the pointer in which the position of the left boundary of the
primary selection is returned. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).
* **`right`** 

Specifies the pointer in which the position of the right boundary of the
primary selection is returned. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns True if the widget owns the primary selection;
otherwise, it returns False.
## RELATED


&cdeman.XmText;.