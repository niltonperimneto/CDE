# XmTextFieldPosToXY
library call`XmTextFieldPosToXY`A TextField function that accesses the x
and y position of a character positionXmTextFieldPosToXYTextField functionsXmTextFieldPosToXY#include <Xm/TextF.h>Boolean`XmTextFieldPosToXY`WidgetwidgetXmTextPositionpositionPosition*xPosition*y
## DESCRIPTION


`XmTextFieldPosToXY`accesses thexandyposition, relative to the upper
left corner of the TextField widget, of a given character position in the
text buffer.

* **`widget`** 

Specifies the TextField widget ID
* **`position`** 

Specifies the character position in the text for which thexandyposition is accessed. This is an integer number of characters
from the beginning of the buffer. The first character position is 0.
* **x** 

Specifies the pointer in which thexposition is returned.
The returned position is the distance from the left side of the widget
to the left border of the character.
This value is meaningful only if the function returns True.
* **y** 

Specifies the pointer in which theyposition is returned.
The returned position is the distance from the top of the widget
to the character's baseline.
This value is meaningful only if the function returns True.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


This function returns True if the character position is displayed in the
TextField widget; otherwise, it returns False, and noxoryvalue is returned.
## RELATED


&cdeman.XmTextField;.