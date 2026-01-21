# XmTextInsertWcs
library call`XmTextInsertWcs`A Text function that inserts a wide character
string into a Text widgetXmTextInsertWcsText functionsXmTextInsertWcs#include <Xm/Text.h>void`XmTextInsertWcs`WidgetwidgetXmTextPositionpositionwchar_t *wcstring
## DESCRIPTION


`XmTextInsertWcs`inserts a wide character string into
the Text widget at a specified location. The character
positions begin at 0 (zero) and are numbered sequentially from
the beginning of the text. For example, to insert a string
after the fourth character, the`position`parameter
must be 4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
If the`XmNcursorPosition`resource is greater than or is the same value as`position`, the`XmNmotionVerifyCallback`is called.

Note that, if`value`is a null string, no callbacks will be
generated, since no modifications will have been made.

* **`widget`** 

Specifies the Text widget ID
* **`position`** 

Specifies the position in the text string where the new character string is
to be inserted
* **`wcstring`** 

Specifies the wide character string value to be added to the Text widget


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextInsert;.