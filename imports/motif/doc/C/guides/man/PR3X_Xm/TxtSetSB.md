# XmTextSetSource
library call`XmTextSetSource`A Text function that sets the source of the widgetXmTextSetSourceText functionsXmTextSetSource#include <Xm/Text.h>void`XmTextSetSource`WidgetwidgetXmTextSourcesourceXmTextPositiontop_characterXmTextPositioncursor_position
## DESCRIPTION


`XmTextSetSource`sets the source of the Text widget. Text
widgets can share sources of text so that editing in one widget is
reflected in another. This function sets the source of one widget
so that it can share the source of another widget.

Setting a new text source destroys the old text source if no other Text
widgets are using that source.
To replace a text source but keep it for later use, create an unmanaged
Text widget and set its source to the text source you want to keep.

* **`widget`** 

Specifies the Text widget ID.
* **`source`** 

Specifies the source with which the widget displays text. This can be a
value returned by the &cdeman.XmTextGetSource; function. If no source
is specified, the widget creates a default string source.
* **`top_character`** 

Specifies the position in the text to display at the top of the widget.
This is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`cursor_position`** 

Specifies the position in the text at which the insert cursor is
located. This is an integer number of characters from the beginning of
the text buffer. The first character position is 0 (zero).


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.