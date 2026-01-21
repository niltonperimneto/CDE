# XmTextPosToXY
library call`XmTextPosToXY`A Text function that accesses the x and y position of a character positionXmTextPosToXYText functionsXmTextPosToXY#include <Xm/Text.h>Boolean`XmTextPosToXY`WidgetwidgetXmTextPositionpositionPosition*xPosition*y
## DESCRIPTION


`XmTextPosToXY`accesses thexandyposition,
relative to the upper
left corner of the Text widget, of a given character position in the
text buffer.

In the case of horizontal writing, the position is the origin of the
character. In the case of vertical writing, the position is the vertical
origin of the character.

* **`widget`** 

Specifies the Text widget ID
* **`position`** 

Specifies the character position in the text for which thexandyposition is accessed. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).
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


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns True if the character position is displayed in the
Text widget; otherwise, it returns False, and noxoryvalue is returned.
## RELATED


&cdeman.XmText;.