# XmTextShowPosition
library call`XmTextShowPosition`A Text function that forces text at a given position to be displayedXmTextShowPositionText functionsXmTextShowPosition#include <Xm/Text.h>void`XmTextShowPosition`WidgetwidgetXmTextPositionposition
## DESCRIPTION


`XmTextShowPosition`forces text at the specified position to be
displayed. If the`XmNautoShowCursorPosition`resource is True, the
application should also set the insert cursor to this position.

* **`widget`** 

Specifies the Text widget ID
* **`position`** 

Specifies the character position to be displayed. This is an integer
number of characters from the beginning of the text buffer. The first
character position is 0 (zero).


If a navigator exists, this function uses the`XmQTnavigator`trait to update the horizontal navigator's value.

For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.