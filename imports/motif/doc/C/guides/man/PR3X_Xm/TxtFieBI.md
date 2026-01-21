# XmTextFieldXYToPos
library call`XmTextFieldXYToPos`A TextField function that accesses the character position nearest an x and y positionXmTextFieldXYToPosTextField functionsXmTextFieldXYToPos#include <Xm/TextF.h>XmTextPosition`XmTextFieldXYToPos`WidgetwidgetPositionxPositiony
## DESCRIPTION


`XmTextFieldXYToPos`accesses the character position nearest to the
specifiedxandyposition, relative to the upper left corner of the
TextField widget.

* **`widget`** 

Specifies the TextField widget ID
* **x** 

Specifies thexposition, relative to the upper left corner of the
widget.
* **y** 

Specifies theyposition, relative to the upper left corner of the
widget.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns the character position in the text nearest thexandyposition specified. This is an integer number of characters
from the beginning of the buffer. The first character position is 0 (zero).
## RELATED


&cdeman.XmTextField;.