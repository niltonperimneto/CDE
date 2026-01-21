# XmTextSetTopCharacter
library call`XmTextSetTopCharacter`A Text function that sets the position of the first character displayedXmTextSetTopCharacterText functionsXmTextSetTopCharacter#include <Xm/Text.h>void`XmTextSetTopCharacter`WidgetwidgetXmTextPositiontop_character
## DESCRIPTION


`XmTextSetTopCharacter`sets the position of the text at the top
of the Text widget.
If the`XmNeditMode`is`XmMULTI_LINE_EDIT`, the line of text
that contains`top_character`is displayed at the top of the widget
without the text shifting left or right.
If the edit mode is`XmSINGLE_LINE_EDIT`, the text moves horizontally
so that`top_character`is the first character displayed.

* **`widget`** 

Specifies the Text widget ID
* **`top_character`** 

Specifies the position in the text to display at the top of the widget.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.