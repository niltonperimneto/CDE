# XmTextFieldSetHighlight
library call`XmTextFieldSetHighlight`A TextField function that highlights textXmTextFieldSetHighlightTextField functionsXmTextFieldSetHighlight#include <Xm/TextF.h>void`XmTextFieldSetHighlight`WidgetwidgetXmTextPositionleftXmTextPositionrightXmHighlightModemode
## DESCRIPTION


`XmTextFieldSetHighlight`highlights text between the two specified
character positions. The`mode`parameter determines the type of
highlighting. Highlighting text merely changes the visual appearance of
the text; it does not set the selection.

* **`widget`** 

Specifies the TextField widget ID
* **`left`** 

Specifies the position of the left boundary of text to be highlighted.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`right`** 

Specifies the position of the right boundary of text to be highlighted.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`mode`** 

Specifies the type of highlighting to be done. A value of`XmHIGHLIGHT_NORMAL`removes highlighting. A value of`XmHIGHLIGHT_SELECTED`highlights the test using reverse video. A
value of`XmHIGHLIGHT_SECONDARY_SELECTED`highlights the text using
underlining.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField;.