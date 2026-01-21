# XmTextScroll
library call`XmTextScroll`A Text function that scrolls textXmTextScrollText functionsXmTextScroll#include <Xm/Text.h>void`XmTextScroll`Widgetwidgetintlines
## DESCRIPTION


`XmTextScroll`scrolls text by a given number
of lines in a Text widget. The sign of the number is interpreted
according to the value of the`XmNlayoutDirection`resource.

* **`widget`** 

Specifies the Text widget ID
* **`lines`** 

Specifies the number of lines of text to scroll. A positive value
causes text to scroll upward; a negative value causes text to scroll
downward.
Note that the text will scroll only as long as one line of text
remains visible in the window.

If a navigator exists, this function uses the`XmQTnavigator`trait to update the vertical navigator's value.

In the case of vertical writing, a positive value causes the text to scroll forward;
a negative value causes the lines to scroll backward.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.