# XmTextGetTopCharacter
library call`XmTextGetTopCharacter`A Text function that accesses the position of the first character displayedXmTextGetTopCharacterText functionsXmTextGetTopCharacter#include <Xm/Text.h>XmTextPosition`XmTextGetTopCharacter`Widgetwidget
## DESCRIPTION


`XmTextGetTopCharacter`accesses the position of the text at the top
of the Text widget.
If there is no text in the Text widget (in other words,`XmNvalue`contains an empty string),
then`XmTextGetTopCharacter`returns 0.

Suppose that the`XmNtopCharacter`resource has been
set to a value greater than the number of characters in
the text widget. In this case,`XmTextGetTopCharacter`returns anXmTextPositionvalue identifying
the position of the first character in the last line of text in a
multiline case; otherwise, it identifies the position of the last
character in the line.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns anXmTextPositionvalue that indicates the state of the`XmNtopCharacter`resource. This is an integer number of characters
from the beginning of the text buffer. The first character position is
0 (zero).
## RELATED


&cdeman.XmText;.