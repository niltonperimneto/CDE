# XmTextSetHighlight
library call`XmTextSetHighlight`A Text function that highlights textXmTextSetHighlightText functionsXmTextSetHighlight#include <Xm/Text.h>void`XmTextSetHighlight`WidgetwidgetXmTextPositionleftXmTextPositionrightXmHighlightModemode
## DESCRIPTION


`XmTextSetHighlight`highlights text between the two specified
character positions. The`mode`parameter determines the type of
highlighting. Highlighting text merely changes the visual appearance of
the text; it does not set the selection.

* **`widget`** 

Specifies the Text widget ID
* **`left`** 

Specifies the position of the left boundary of text to be highlighted.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`right`** 

Specifies the position of the right boundary of text to be highlighted.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`mode`** 

Specifies the type of highlighting to be done. A value of`XmHIGHLIGHT_NORMAL`removes highlighting. A value of`XmHIGHLIGHT_SELECTED`highlights the text using reverse video. A
value of`XmHIGHLIGHT_SECONDARY_SELECTED`highlights the text using
underlining.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.