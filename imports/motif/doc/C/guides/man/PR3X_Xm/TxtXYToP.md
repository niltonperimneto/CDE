# XmTextXYToPos
library call`XmTextXYToPos`A Text function that accesses the character position nearest an x and y positionXmTextXYToPosText functionsXmTextXYToPos#include <Xm/Text.h>XmTextPosition`XmTextXYToPos`WidgetwidgetPositionxPositiony
## DESCRIPTION


`XmTextXYToPos`accesses the character position nearest to the
specifiedxandyposition, relative to the upper left corner of the
Text widget.

In the case of horizontal writing, the position is the origin of the character.
In the case of vertical writing, the position is the vertical origin of
the character.

* **`widget`** 

Specifies the Text widget ID
* **x** 

Specifies thexposition, relative to the upper left corner of the
widget
* **y** 

Specifies theyposition, relative to the upper left corner of the
widget


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns the character position in the text nearest thexandyposition specified. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).
## RELATED


&cdeman.XmText;.